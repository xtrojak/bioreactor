use crate::test_bioreactor::TestReactorConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Hub server configuration data.
///
/// At the moment, it only includes:
///  - Basic metadata (`name` and optional `description`).
///  - "Secret" `server_password` used for token generation.
///  - "Public" `user_password` vector used to check during authentication.
///
/// Later, it will also incorporate data about which devices should the hub connect to and
/// what method of connection should be used.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubConfig {
    pub name: String,
    pub description: Option<String>,
    pub server_password: String,
    pub user_password: Vec<String>,
    pub device_config: Vec<DeviceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DeviceConfig {
    TestDevice {
        id: String,
        name: String,
        description: Option<String>,
    },
    NetworkDevice {
        // TODO: Add some fancy config options :)
    },
}

impl HubConfig {
    /// Utility function to read config data from a JSON file supplied in the
    /// first program argument.
    pub fn read_from_args() -> Result<HubConfig, String> {
        let args = std::env::args().collect::<Vec<_>>();
        let config_path_str = args.get(1).cloned().unwrap_or_else(|| {
            eprintln!("!! Config file path not found. Using `hub.json` as default.");
            "hub.json".to_string()
        });
        let config_path = Path::new(&config_path_str);

        if !config_path.exists() {
            return Err("!! Config file not found. Aborting.".to_string());
        }

        let config_str = std::fs::read_to_string(&config_path).map_err(|_| {
            format!(
                "!! Cannot read configuration file from `{}`.",
                config_path_str
            )
        })?;

        Self::from_json(&config_str)
            .map_err(|e| format!("!! Format in `{}` invalid: {:?}", config_path_str, e))
    }

    /// Deserialize the config from a JSON string.
    pub fn from_json(json: &str) -> Result<HubConfig, serde_json::Error> {
        serde_json::from_str::<HubConfig>(json)
    }

    /// Check whether the config does not have some undesired problems.
    pub fn validate(&self) -> Result<(), String> {
        if self.server_password.len() < 16 {
            return Err("Server password must be at least 16 characters long.".to_string());
        }
        Ok(())
    }
}

impl DeviceConfig {
    /// Check if this device config corresponds to a test device config.
    pub fn is_test_device(&self) -> bool {
        match self {
            DeviceConfig::TestDevice { .. } => true,
            _ => false,
        }
    }

    /// Convert this device config into a test device config.
    pub fn to_test_device(self) -> Option<TestReactorConfig> {
        match self {
            DeviceConfig::TestDevice {
                id,
                name,
                description,
            } => Some(TestReactorConfig {
                id,
                name,
                description,
            }),
            _ => None,
        }
    }
}
