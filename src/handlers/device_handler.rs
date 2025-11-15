use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

use crate::dto::{DeviceCreateRequest, DeviceResponse};
use crate::service::DeviceService;

/// HTTP handler to create a new device.
/// Calls DeviceService::create with payload data, returns JSON DeviceResponse on success.
pub async fn create_device(
    Extension(service): Extension<std::sync::Arc<DeviceService>>,
    Json(payload): Json<DeviceCreateRequest>,
) -> impl IntoResponse {
    match service
        .create(payload.name, payload.board_id, payload.board_type, payload.project_path)
        .await
    {
        Ok(device) => (StatusCode::CREATED, Json(DeviceResponse::from(&device))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to create device: {}", e),
        )
            .into_response(),
    }
}

/// HTTP handler to retrieve a device by ID.
/// Parses UUID from path, calls DeviceService::get, handles not-found and errors.
pub async fn get_device(
    Extension(service): Extension<std::sync::Arc<DeviceService>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let parsed = Uuid::parse_str(&id);
    if let Err(_) = parsed {
        return (StatusCode::BAD_REQUEST, "invalid uuid").into_response();
    }
    let id = parsed.unwrap();

    match service.get(id).await {
        Ok(Some(device)) => (StatusCode::OK, Json(DeviceResponse::from(&device))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to find device: {}", e),
        )
            .into_response(),
    }
}

/// HTTP handler to list all devices.
/// Calls DeviceService::list, returns JSON array of DeviceResponse on success.
pub async fn list_devices(
    Extension(service): Extension<std::sync::Arc<DeviceService>>,
) -> impl IntoResponse {
    match service.list().await {
        Ok(list) => (
            StatusCode::OK,
            Json(
                list.iter()
                    .map(|d| DeviceResponse::from(d))
                    .collect::<Vec<_>>(),
            ),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to list devices: {}", e),
        )
            .into_response(),
    }
}
