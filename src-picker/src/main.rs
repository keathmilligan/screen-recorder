//! Screen Recorder Picker - Headless xdg-desktop-portal backend.
//!
//! This service runs as a systemd user service and handles screencast requests
//! from xdg-desktop-portal. Instead of showing a picker UI, it queries the main
//! screen-recorder app via IPC for the user's capture selection and auto-approves.
//!
//! # Architecture
//!
//! ```text
//! Main App (Tauri)          This Service                xdg-desktop-portal
//!       |                        |                             |
//!       |                        |<--- ScreenCast request -----|
//!       |<-- IPC: query_selection|                             |
//!       |--- IPC: selection ---->|                             |
//!       |                        |--- auto-approve ----------->|
//!       |                        |                             |
//! ```

mod ipc_client;
mod portal_backend;

use tracing::info;
use tracing_subscriber::EnvFilter;
use zbus::connection::Builder;

const SERVICE_NAME: &str = "org.freedesktop.impl.portal.desktop.screenrecorder";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("screen_recorder_picker=debug".parse()?)
                .add_directive("zbus=warn".parse()?),
        )
        .init();

    info!("Starting screen-recorder-picker service");
    info!("IPC socket path: {:?}", ipc_client::get_socket_path());

    // Build D-Bus connection and request the service name
    let conn = Builder::session()?
        .name(SERVICE_NAME)?
        .build()
        .await?;

    info!("Connected to D-Bus session bus");
    info!("Registered service name: {}", SERVICE_NAME);

    // Register the portal backend interface
    portal_backend::register_portal_backend(&conn).await?;

    info!("Portal backend ready - waiting for requests");

    // Keep the service running
    // In production, this would be managed by systemd
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
