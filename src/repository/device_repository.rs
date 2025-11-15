use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Device;

/*
* Repository & Adapters
- `DeviceRepository` is an async trait defining operations like `create`, `find_by_id`, and `list`.
- `InMemoryDeviceRepository` implements this using a thread-safe `RwLock<HashMap<Uuid, Device>>` for concurrent access.
*/
/// Trait for device persistence operations. Implementations handle storing and retrieving Device entities.
/// Requires Send + Sync for async compatibility.
#[async_trait]
pub trait DeviceRepository: Send + Sync {
    /// Persists a new Device and returns it.
    async fn create(&self, device: Device) -> Result<Device>;
    /// Retrieves a Device by its UUID, if it exists.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Device>>;
    /// Retrieves all persisted Devices.
    async fn list(&self) -> Result<Vec<Device>>;
}
