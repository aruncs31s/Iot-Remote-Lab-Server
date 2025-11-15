use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use tower_http::trace::TraceLayer;

use iot_remote_lab_server::adapters::InMemoryDeviceRepository;
use iot_remote_lab_server::handlers::{
    build_firmware, clean_project, create_basic_main, create_device, get_device, init_project,
    list_devices, upload_firmware,
};
use iot_remote_lab_server::service::{DeviceService, PlatformIOService};

/// Entry point of the application. Initializes services, checks for PlatformIO installation,
/// sets up routes, and starts the HTTP server on 127.0.0.1:3000.
#[tokio::main]
async fn main() {
    // repository adapter (in-memory for demo)
    let repo = InMemoryDeviceRepository::new();
    let device_service = Arc::new(DeviceService::new(Arc::new(repo)));
    let pio_service = Arc::new(PlatformIOService::new());

    // Check if PlatformIO is available
    match std::process::Command::new("platformio")
        .arg("--version")
        .output()
    {
        Ok(output) if output.status.success() => {
            println!("PlatformIO is available");
        }
        _ => {
            eprintln!("Warning: PlatformIO not found. ESP32 operations will fail. Please install PlatformIO: https://platformio.org/install");
        }
    }

    let app = register_routes(device_service.clone(), pio_service.clone())
        .layer(Extension(device_service))
        .layer(Extension(pio_service))
        .layer(TraceLayer::new_for_http());
    println!("Listening on http://127.0.0.1:3000");

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
/// Defines and returns the Axum router with all API routes configured.
/// Routes include device CRUD and ESP32 operations, with services injected via Extension.
fn register_routes(
    device_service: Arc<DeviceService>,
    pio_service: Arc<PlatformIOService>,
) -> Router {
    Router::new()
        .route("/devices", post(create_device).get(list_devices))
        .route("/devices/:id", get(get_device))
        .route("/devices/:id/build", post(build_firmware))
        .route("/devices/:id/upload", post(upload_firmware))
        .route("/devices/:id/init", post(init_project))
        .route("/devices/:id/clean", post(clean_project))
        .route("/devices/:id/create-main", post(create_basic_main))
}
