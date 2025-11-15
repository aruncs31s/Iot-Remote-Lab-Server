use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::Result;
use uuid::Uuid;

use crate::domain::Device;
use crate::repository::DeviceRepository;

/// In-memory implementation of DeviceRepository using a thread-safe HashMap.
#[derive(Clone, Default)]
pub struct InMemoryDeviceRepository {
    // Shared, concurrent map
    store: Arc<RwLock<HashMap<Uuid, Device>>>,
}

impl InMemoryDeviceRepository {
    /// Constructor for the in-memory repository.
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl DeviceRepository for InMemoryDeviceRepository {
    /// Stores a Device in the in-memory map.
    async fn create(&self, device: Device) -> Result<Device> {
        let mut w = self.store.write().await;
        w.insert(device.id, device.clone());
        Ok(device)
    }

    /// Looks up a Device by ID in the map.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Device>> {
        let r = self.store.read().await;
        Ok(r.get(&id).cloned())
    }

    /// Returns all stored Devices as a vector.
    async fn list(&self) -> Result<Vec<Device>> {
        let r = self.store.read().await;
        Ok(r.values().cloned().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio_test::block_on;

    /// Basic test for repository operations: create, find, and list.
    #[test]
    fn basic_repo() {
        let repo = InMemoryDeviceRepository::new();
        let device = Device::new("d1");
        let created = block_on(repo.create(device.clone())).unwrap();
        assert_eq!(created, device);
        let found = block_on(repo.find_by_id(device.id)).unwrap().unwrap();
        assert_eq!(found, device);
        let list = block_on(repo.list()).unwrap();
        assert_eq!(list.len(), 1);
    }
}
