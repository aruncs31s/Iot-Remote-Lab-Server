use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub board_id: String,
    pub board_type: Option<String>, // ESP32 board type (e.g., "esp32dev", "esp32-s3-devkitc-1")
    pub project_path: Option<String>, // Path to PlatformIO project directory
}

impl Device {
    /// Constructor for a basic `Device` with a generated UUID and no ESP32-specific config.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            board_id: String::new(),
            board_type: None,
            project_path: None,
        }
    }

    pub fn with_esp32_config(
        name: impl Into<String>,
        board_id: String,
        board_type: String,
        project_path: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            board_id,
            board_type: Some(board_type),
            project_path: Some(project_path),
        }
    }
}
