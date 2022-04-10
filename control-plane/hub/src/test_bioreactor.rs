use crate::config::DeviceMetadata;
use crate::device::{Bioreactor, DeviceStatus};

/// A dummy "virtual" bioreactor device that can be used for testing.
pub struct TestReactor {
    metadata: DeviceMetadata,
    status: DeviceStatus,
}

#[derive(Clone, Debug)]
pub struct TestReactorConfig {
    pub meta: DeviceMetadata,
}

impl TestReactor {
    /// Create a `TestReactor` from a `TestReactorConfig` instance.
    pub fn from_config(config: TestReactorConfig) -> TestReactor {
        TestReactor {
            metadata: config.meta,
            status: DeviceStatus::Inactive,
        }
    }
}

impl Bioreactor for TestReactor {
    fn get_id(&self) -> String {
        self.metadata.id.clone()
    }

    fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    fn get_description(&self) -> Option<String> {
        self.metadata.description.clone()
    }

    fn get_current_status(&self) -> DeviceStatus {
        self.status
    }

    fn get_current_status_extra(&self) -> Option<String> {
        None
    }

    fn get_target_status(&self) -> DeviceStatus {
        self.status
    }

    fn set_target_status(&mut self, status: DeviceStatus) -> Result<(), String> {
        if status == DeviceStatus::Offline {
            Err("Setting device status to `Offline` is not permitted.".to_string())
        } else {
            self.status = status;
            Ok(())
        }
    }
}
