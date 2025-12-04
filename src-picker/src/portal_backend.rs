//! Portal backend implementing org.freedesktop.impl.portal.ScreenCast.
//!
//! This module implements the D-Bus interface that xdg-desktop-portal calls
//! when applications request screen capture.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use zbus::{interface, Connection};
use zbus::zvariant::{ObjectPath, OwnedValue, Value};

use crate::ipc_client::{query_selection, IpcResponse};

/// Session state tracked by the portal backend.
#[derive(Debug, Clone)]
pub struct Session {
    /// Source types requested (1=monitor, 2=window, 4=virtual)
    pub source_types: u32,
    /// Whether cursor should be included
    pub cursor_mode: u32,
    /// Whether to persist authorization
    pub persist_mode: u32,
    /// Restore token if provided
    pub restore_token: Option<String>,
}

/// Portal backend state shared across D-Bus handlers.
#[derive(Debug, Default)]
pub struct PortalState {
    /// Active sessions by handle
    pub sessions: HashMap<String, Session>,
}

/// The ScreenCast portal backend implementation.
pub struct ScreenCastBackend {
    state: Arc<Mutex<PortalState>>,
}

impl ScreenCastBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PortalState::default())),
        }
    }
}

impl Default for ScreenCastBackend {
    fn default() -> Self {
        Self::new()
    }
}

/// Response codes for portal methods.
/// 0 = Success, 1 = Cancelled by user, 2 = Other error
const PORTAL_RESPONSE_SUCCESS: u32 = 0;
const PORTAL_RESPONSE_CANCELLED: u32 = 1;

/// Source type flags
const SOURCE_TYPE_MONITOR: u32 = 1;
const SOURCE_TYPE_WINDOW: u32 = 2;

/// Helper to extract u32 from OwnedValue
fn get_u32(options: &HashMap<String, OwnedValue>, key: &str) -> Option<u32> {
    options.get(key).and_then(|v| {
        match v.downcast_ref::<Value>() {
            Ok(Value::U32(val)) => Some(val),
            _ => None,
        }
    })
}

/// Helper to extract String from OwnedValue
fn get_string(options: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    options.get(key).and_then(|v| {
        match v.downcast_ref::<Value>() {
            Ok(Value::Str(s)) => Some(s.to_string()),
            _ => None,
        }
    })
}

#[interface(name = "org.freedesktop.impl.portal.ScreenCast")]
impl ScreenCastBackend {
    /// Available source types (monitors and windows)
    #[zbus(property)]
    async fn available_source_types(&self) -> u32 {
        SOURCE_TYPE_MONITOR | SOURCE_TYPE_WINDOW
    }

    /// Available cursor modes (hidden=1, embedded=2, metadata=4)
    /// We support embedded cursor (drawn into frame)
    #[zbus(property)]
    async fn available_cursor_modes(&self) -> u32 {
        2 // embedded only
    }

    /// Portal interface version
    #[zbus(property)]
    async fn version(&self) -> u32 {
        4
    }

    /// Create a new screencast session.
    ///
    /// Called by xdg-desktop-portal when an app calls CreateSession.
    async fn create_session(
        &self,
        handle: ObjectPath<'_>,
        session_handle: ObjectPath<'_>,
        _app_id: &str,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)> {
        info!(
            "CreateSession: handle={}, session={}",
            handle.as_str(),
            session_handle.as_str()
        );

        let session = Session {
            source_types: 0,
            cursor_mode: 2, // embedded
            persist_mode: 0,
            restore_token: None,
        };

        {
            let mut state = self.state.lock().await;
            state.sessions.insert(session_handle.to_string(), session);
        }

        let results: HashMap<String, OwnedValue> = HashMap::new();
        Ok((PORTAL_RESPONSE_SUCCESS, results))
    }

    /// Select capture sources.
    ///
    /// Called by xdg-desktop-portal when an app calls SelectSources.
    async fn select_sources(
        &self,
        handle: ObjectPath<'_>,
        session_handle: ObjectPath<'_>,
        _app_id: &str,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)> {
        info!(
            "SelectSources: handle={}, session={}",
            handle.as_str(),
            session_handle.as_str()
        );

        // Extract options
        let source_types = get_u32(&options, "types")
            .unwrap_or(SOURCE_TYPE_MONITOR | SOURCE_TYPE_WINDOW);
        let cursor_mode = get_u32(&options, "cursor_mode").unwrap_or(2);
        let persist_mode = get_u32(&options, "persist_mode").unwrap_or(0);
        let restore_token = get_string(&options, "restore_token");

        info!(
            "SelectSources options: types={}, cursor_mode={}, persist_mode={}, restore_token={:?}",
            source_types, cursor_mode, persist_mode, restore_token
        );

        // Update session state
        {
            let mut state = self.state.lock().await;
            if let Some(session) = state.sessions.get_mut(session_handle.as_str()) {
                session.source_types = source_types;
                session.cursor_mode = cursor_mode;
                session.persist_mode = persist_mode;
                session.restore_token = restore_token;
            } else {
                warn!("SelectSources: session not found: {}", session_handle.as_str());
            }
        }

        let results: HashMap<String, OwnedValue> = HashMap::new();
        Ok((PORTAL_RESPONSE_SUCCESS, results))
    }

    /// Start the screencast stream.
    ///
    /// This is where we query the main app for the selection and auto-approve.
    async fn start(
        &self,
        handle: ObjectPath<'_>,
        session_handle: ObjectPath<'_>,
        _app_id: &str,
        _parent_window: &str,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)> {
        info!(
            "Start: handle={}, session={}",
            handle.as_str(),
            session_handle.as_str()
        );

        // Query the main app for the current selection
        let selection = match query_selection().await {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to query main app: {}", e);
                // Return cancelled - main app not available
                return Ok((PORTAL_RESPONSE_CANCELLED, HashMap::new()));
            }
        };

        match selection {
            IpcResponse::Selection {
                source_type,
                source_id,
                geometry,
            } => {
                info!(
                    "Got selection from main app: type={}, id={}, geometry={:?}",
                    source_type, source_id, geometry
                );

                // Build the streams array for the portal response
                // The actual PipeWire node ID would come from the compositor
                // For now, we return a placeholder that tells the portal what to capture
                let mut stream_properties: HashMap<String, OwnedValue> = HashMap::new();

                // Determine source type for portal
                let portal_source_type: u32 = match source_type.as_str() {
                    "monitor" => SOURCE_TYPE_MONITOR,
                    "window" => SOURCE_TYPE_WINDOW,
                    "region" => SOURCE_TYPE_MONITOR, // Region captures full monitor, app crops
                    _ => SOURCE_TYPE_MONITOR,
                };

                stream_properties.insert(
                    "source_type".to_string(),
                    OwnedValue::from(portal_source_type),
                );

                // Add source identifier
                // Note: The actual mapping to PipeWire node happens via the compositor
                // We provide the identifier so the portal knows which source to use
                stream_properties.insert(
                    "id".to_string(),
                    OwnedValue::try_from(Value::new(&source_id)).unwrap_or_else(|_| OwnedValue::from(0u32)),
                );

                if let Some(geom) = geometry {
                    // For region capture, include geometry info as structure
                    // Portal expects (i32, i32) tuples
                    stream_properties.insert(
                        "position".to_string(),
                        OwnedValue::try_from(Value::new((geom.x, geom.y)))
                            .unwrap_or_else(|_| OwnedValue::from(0u32)),
                    );
                    stream_properties.insert(
                        "size".to_string(),
                        OwnedValue::try_from(Value::new((geom.width as i32, geom.height as i32)))
                            .unwrap_or_else(|_| OwnedValue::from(0u32)),
                    );
                }

                // The streams array: each element is (node_id, properties)
                // node_id of 0 means "use the identified source"
                // The portal/compositor will resolve this to the actual PipeWire node
                let streams: Vec<(u32, HashMap<String, OwnedValue>)> =
                    vec![(0, stream_properties)];

                let mut results: HashMap<String, OwnedValue> = HashMap::new();
                results.insert(
                    "streams".to_string(),
                    OwnedValue::try_from(Value::new(streams)).unwrap_or_else(|_| OwnedValue::from(0u32)),
                );

                Ok((PORTAL_RESPONSE_SUCCESS, results))
            }
            IpcResponse::NoSelection => {
                warn!("No selection available from main app");
                Ok((PORTAL_RESPONSE_CANCELLED, HashMap::new()))
            }
            IpcResponse::Error { message } => {
                error!("Error from main app: {}", message);
                Ok((PORTAL_RESPONSE_CANCELLED, HashMap::new()))
            }
        }
    }
}

/// Register the portal backend on the D-Bus session bus.
pub async fn register_portal_backend(conn: &Connection) -> zbus::Result<()> {
    let backend = ScreenCastBackend::new();

    conn.object_server()
        .at("/org/freedesktop/portal/desktop", backend)
        .await?;

    info!("Portal backend registered at /org/freedesktop/portal/desktop");
    Ok(())
}
