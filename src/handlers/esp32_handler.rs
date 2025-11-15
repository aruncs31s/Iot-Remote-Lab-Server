use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

use crate::dto::{BuildRequest, CommandResponse, InitProjectRequest, UploadRequest};
use crate::service::{DeviceService, PlatformIOService};

/// HTTP handler to build firmware for a device.
/// Fetches the device, validates project path, calls PlatformIOService::build_project.
pub async fn build_firmware(
    Extension(device_service): Extension<std::sync::Arc<DeviceService>>,
    Extension(pio_service): Extension<std::sync::Arc<PlatformIOService>>,
    Json(payload): Json<BuildRequest>,
) -> impl IntoResponse {
    // Get device
    let device = match device_service.get(payload.device_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device not found".to_string()),
                }),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some(format!("Failed to get device: {}", e)),
                }),
            )
                .into_response()
        }
    };

    // Check if device has project path
    let project_path = match device.project_path {
        Some(p) => p,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device has no project path configured".to_string()),
                }),
            )
                .into_response()
        }
    };

    // Build project
    match pio_service.build_project(&project_path).await {
        Ok(output) => (
            StatusCode::OK,
            Json(CommandResponse {
                success: true,
                output,
                error: None,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CommandResponse {
                success: false,
                output: "".to_string(),
                error: Some(format!("Build failed: {}", e)),
            }),
        )
            .into_response(),
    }
}

/// HTTP handler to upload firmware to a device.
/// Fetches the device, validates project path, calls PlatformIOService::upload_firmware.
pub async fn upload_firmware(
    Extension(device_service): Extension<std::sync::Arc<DeviceService>>,
    Extension(pio_service): Extension<std::sync::Arc<PlatformIOService>>,
    Json(payload): Json<UploadRequest>,
) -> impl IntoResponse {
    // Get device
    let device = match device_service.get(payload.device_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device not found".to_string()),
                }),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some(format!("Failed to get device: {}", e)),
                }),
            )
                .into_response()
        }
    };

    // Check if device has project path
    let project_path = match device.project_path {
        Some(p) => p,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device has no project path configured".to_string()),
                }),
            )
                .into_response()
        }
    };

    // Upload firmware
    match pio_service
        .upload_firmware(&project_path, payload.port.as_deref())
        .await
    {
        Ok(output) => (
            StatusCode::OK,
            Json(CommandResponse {
                success: true,
                output,
                error: None,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CommandResponse {
                success: false,
                output: "".to_string(),
                error: Some(format!("Upload failed: {}", e)),
            }),
        )
            .into_response(),
    }
}

/// HTTP handler to initialize a PlatformIO project for a device.
/// Fetches the device, validates project path, calls PlatformIOService::init_project.
pub async fn init_project(
    Extension(device_service): Extension<std::sync::Arc<DeviceService>>,
    Extension(pio_service): Extension<std::sync::Arc<PlatformIOService>>,
    Json(payload): Json<InitProjectRequest>,
) -> impl IntoResponse {
    // Get device
    let device = match device_service.get(payload.device_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device not found".to_string()),
                }),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some(format!("Failed to get device: {}", e)),
                }),
            )
                .into_response()
        }
    };

    // Check if device has project path
    let project_path = match device.project_path {
        Some(p) => p,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device has no project path configured".to_string()),
                }),
            )
                .into_response()
        }
    };

    // Initialize project
    match pio_service
        .init_project(&project_path, &payload.board)
        .await
    {
        Ok(output) => (
            StatusCode::OK,
            Json(CommandResponse {
                success: true,
                output,
                error: None,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CommandResponse {
                success: false,
                output: "".to_string(),
                error: Some(format!("Project initialization failed: {}", e)),
            }),
        )
            .into_response(),
    }
}

/// HTTP handler to create a basic main.cpp for a device.
/// Parses UUID from path, fetches device, validates project path, calls PlatformIOService::create_basic_main.
pub async fn create_basic_main(
    Extension(device_service): Extension<std::sync::Arc<DeviceService>>,
    Extension(pio_service): Extension<std::sync::Arc<PlatformIOService>>,
    axum::extract::Path(device_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let parsed = Uuid::parse_str(&device_id);
    if let Err(_) = parsed {
        return (
            StatusCode::BAD_REQUEST,
            Json(CommandResponse {
                success: false,
                output: "".to_string(),
                error: Some("Invalid device ID".to_string()),
            }),
        )
            .into_response();
    }
    let device_id = parsed.unwrap();

    // Get device
    let device = match device_service.get(device_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device not found".to_string()),
                }),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some(format!("Failed to get device: {}", e)),
                }),
            )
                .into_response()
        }
    };

    // Check if device has project path
    let project_path = match device.project_path {
        Some(p) => p,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device has no project path configured".to_string()),
                }),
            )
                .into_response()
        }
    };

    // Create basic main file
    match pio_service.create_basic_main(&project_path).await {
        Ok(_) => (
            StatusCode::OK,
            Json(CommandResponse {
                success: true,
                output: "Basic main.cpp created successfully".to_string(),
                error: None,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CommandResponse {
                success: false,
                output: "".to_string(),
                error: Some(format!("Failed to create main file: {}", e)),
            }),
        )
            .into_response(),
    }
}

/// HTTP handler to clean build files for a device.
/// Parses UUID from path, fetches device, validates project path, calls PlatformIOService::clean_project.
pub async fn clean_project(
    Extension(device_service): Extension<std::sync::Arc<DeviceService>>,
    Extension(pio_service): Extension<std::sync::Arc<PlatformIOService>>,
    axum::extract::Path(device_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let parsed = Uuid::parse_str(&device_id);
    if let Err(_) = parsed {
        return (
            StatusCode::BAD_REQUEST,
            Json(CommandResponse {
                success: false,
                output: "".to_string(),
                error: Some("Invalid device ID".to_string()),
            }),
        )
            .into_response();
    }
    let device_id = parsed.unwrap();

    // Get device
    let device = match device_service.get(device_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device not found".to_string()),
                }),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some(format!("Failed to get device: {}", e)),
                }),
            )
                .into_response()
        }
    };

    // Check if device has project path
    let project_path = match device.project_path {
        Some(p) => p,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CommandResponse {
                    success: false,
                    output: "".to_string(),
                    error: Some("Device has no project path configured".to_string()),
                }),
            )
                .into_response()
        }
    };

    // Clean project
    match pio_service.clean_project(&project_path).await {
        Ok(output) => (
            StatusCode::OK,
            Json(CommandResponse {
                success: true,
                output,
                error: None,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CommandResponse {
                success: false,
                output: "".to_string(),
                error: Some(format!("Clean failed: {}", e)),
            }),
        )
            .into_response(),
    }
}
