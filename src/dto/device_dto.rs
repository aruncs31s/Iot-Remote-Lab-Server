use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::Device;

// DTO for creating a new Device via API request.
// Prior to this , a list containing available board types should be fetched from the server.
#[derive(Debug, Deserialize)]
pub struct DeviceCreateRequest {
    pub name: String,
    pub board_type: Option<String>,
    pub board_id: String,
    pub project_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub name: String,
    pub board_type: Option<String>,
    pub board_id: String,
    pub project_path: Option<String>,
}

/// Converts a Device entity to a DeviceResponse DTO for JSON serialization.
impl From<&Device> for DeviceResponse {
    fn from(d: &Device) -> Self {
        DeviceResponse {
            id: d.id,
            board_id: d.board_id.clone(),
            name: d.name.clone(),
            board_type: d.board_type.clone(),
            project_path: d.project_path.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BuildRequest {
    pub device_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UploadRequest {
    pub device_id: Uuid,
    pub port: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InitProjectRequest {
    pub device_id: Uuid,
    pub board: String,
}

#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}
