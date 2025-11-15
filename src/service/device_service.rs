use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::domain::Device;
use crate::repository::DeviceRepository;

#[derive(Clone)]
pub struct DeviceService {
    repository: Arc<dyn DeviceRepository + Send + Sync>,
}

impl DeviceService {
    /// Constructor for DeviceService, injecting the repository dependency.
    pub fn new(repository: Arc<dyn DeviceRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        board_id: String,
        board_type: Option<String>,
        project_path: Option<String>,
    ) -> Result<Device> {
        let device = if let (Some(board), Some(path)) = (board_type, project_path) {
            Device::with_esp32_config(
                name,
                 board_id,
                 board,
                  path,
                )
        } else {
            Device::new(name)
        };
        self.repository.create(device.clone()).await
    }

    /// Retrieves a Device by ID via the repository.
    pub async fn get(&self, id: Uuid) -> Result<Option<Device>> {
        self.repository.find_by_id(id).await
    }

    /// Lists all Devices via the repository.
    pub async fn list(&self) -> Result<Vec<Device>> {
        self.repository.list().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::InMemoryDeviceRepository;
    use tokio_test::block_on;

    /// Test for creating a device and retrieving it.
    #[test]
    fn create_and_get() {
        let repo = InMemoryDeviceRepository::new();
        let service = DeviceService::new(Arc::new(repo));
        let created = block_on(service.create("my-device", "board-id-123".to_string(), None::<String>, None::<String>)).unwrap();
        let got = block_on(service.get(created.id)).unwrap().unwrap();
        assert_eq!(got.name, "my-device");
    }
}
