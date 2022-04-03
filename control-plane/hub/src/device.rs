use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// An alias for a standalone, dynamically typed `Bioreactor`. In most cases, the instances
/// of the `Bioreactor` trait will actually be stored using this type.
pub type BioreactorDevice = Box<dyn Bioreactor + Send + Sync>;

/// Enumeration of possible "high-level" bioreactor states. A device can be `Offline`,
/// which means that a connection is broken and the device cannot be reached. If connection
/// is established, a device can be `Inactive`, which means that all instruments are turned off
/// and no logging or monitoring is taking place. Finally, an `Active` state indicates that
/// the device is working.
///
/// Generally, one should be able to change device status from `Active` to `Inactive`
/// or vice-versa, but the `Offline` status is reserved as an indicator of device communication
/// error. It is automatically resolved once the device is properly reconnected. Also, changing
/// the status can take some time, as instruments have to be activated/deactivated.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceStatus {
    Offline,
    Inactive,
    Active,
}

/// A trait implemented by individual bioreactor devices. Additional functionality will
/// be added as we identify what else is necessary.
///
/// Since the objects will be typically used in async/multi-threaded environment, we mostly
/// assume that results of individual operations are copied instead of borrowed. This makes
/// it slightly easier to implement the trait as one does not need to be concerned with
/// ownership.
pub trait Bioreactor {
    /// Get a unique device ID (usually assigned in configuration).
    fn get_id(&self) -> String;

    /// Get a human-readable device name (usually assigned in configuration).
    fn get_name(&self) -> String;

    /// Get a human-readable, optional device description (usually assigned in configuration).
    fn get_description(&self) -> Option<String>;

    /// Get current device status.
    fn get_current_status(&self) -> DeviceStatus;

    /// Get a device status that is currently being targeted.
    fn get_target_status(&self) -> DeviceStatus;

    /// Try to update the desired device status.
    fn set_target_status(&mut self, status: DeviceStatus) -> Result<(), String>;
}

/// `DeviceDepot` serves as a thread-safe storage for individual bioreactor devices.
/// The assumption is that the list of devices is immutable at runtime, and each device is
/// guarded by a read-write lock.
///
/// To retrieve device info, a read lock should be sufficient, while write lock is necessary
/// to modify the device. However, internal device instruments will be most likely guarded
/// by other, more fine-grained locks to ensure multiple requests can operate on different
/// instruments simultaneously. However, note that these operations typically still have to be
/// serialised using some communication bus going directly to the device.
pub struct DeviceDepot {
    devices: HashMap<String, RwLock<BioreactorDevice>>,
}

impl DeviceDepot {
    /// Wrap a collection of `Bioreactor` devices into a `DeviceDepot`.
    pub fn new(devices: Vec<BioreactorDevice>) -> DeviceDepot {
        // TODO: Test that device IDs are unique.
        let devices = devices
            .into_iter()
            .map(|device| (device.get_id(), RwLock::new(device)))
            .collect();
        DeviceDepot { devices }
    }

    /// Get a vector of all device IDs managed in this collection.
    pub fn device_ids(&self) -> Vec<String> {
        self.devices.iter().map(|(k, _)| k.clone()).collect()
    }

    /// Get a read-only reference to a particular device (identified using its `id`),
    /// assuming such a device exists.
    ///
    /// WARNING: A single asynchronous task should only hold one locked reference to
    /// the same `Bioreactor`. Interleaving read/write/read locks in the same thread
    /// can cause a deadlock.
    pub async fn get_device(&self, id: &str) -> Option<RwLockReadGuard<'_, BioreactorDevice>> {
        if let Some(lock) = self.devices.get(id) {
            Some(lock.read().await)
        } else {
            None
        }
    }

    /// Get a mutable reference to a particular device (identified using its `id`),
    /// assuming such a device exists.
    ///
    /// WARNING: A single asynchronous task should only hold one locked reference to
    /// the same `Bioreactor`. Interleaving read/write/read locks in the same thread
    /// can cause a deadlock.
    pub async fn get_device_mut(&self, id: &str) -> Option<RwLockWriteGuard<'_, BioreactorDevice>> {
        if let Some(lock) = self.devices.get(id) {
            Some(lock.write().await)
        } else {
            None
        }
    }
}
