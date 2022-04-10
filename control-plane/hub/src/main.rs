#[macro_use]
extern crate rocket;

use crate::config::HubConfig;
use crate::device::{BioreactorDevice, DeviceDepot};
use crate::network_bioreactor::NetworkReactor;
use crate::test_bioreactor::TestReactor;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use std::process::exit;

/// Module where all authentication code resides. In particular, it defines `/login` and `/renew`
/// endpoints. It also provides `ApiToken` request guard that can be used to enforce authentication
/// on other endpoints.
mod api_auth;

/// Module where device-related endpoints are defined. In particular, device list (`[GET] /hub`)
/// device info (`[GET] /hub/<device-id>`), and device update (`[POST] /hub/<device-id>`).
mod api_device;

/// Describes configuration of the hub server, including deserialization from a JSON file.
mod config;

/// Code related to management of individual bioreactor devices. In particular, it defines
/// the `Bioreactor` trait that needs to be implemented by structures that manage access to
/// a bioreactor device. It also defines `DeviceDepot` which is a thread-safe collection for
/// accessing `Bioreactors` as part of the server state.
mod device;

/// Defines a "virtual" bioreactor device that can be used for testing.
mod test_bioreactor;

/// Defines a network device that uses a low-level protocol to connect to a real
/// (or a simulated) bioreactor device.
///
/// For now, each such device operates on a separate port. Later, we may re-architect
/// this to allow a shared `TcpListener` across devices, but for now it is not really
/// useful.
mod network_bioreactor;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load configuration from the first program argument.
    let config = match HubConfig::read_from_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    // Sanity check the configuration.
    if let Err(e) = config.validate() {
        eprintln!("!! Provided config is not valid.");
        eprintln!("!! {}", e);
        exit(1);
    }

    let mut devices: Vec<BioreactorDevice> = Vec::new();

    for device in config.device_config.clone() {
        if let Some(config) = device.to_test_device() {
            devices.push(Box::new(TestReactor::from_config(config)));
        }
        if let Some(config) = device.to_network_device() {
            devices.push(Box::new(NetworkReactor::from_config(config)));
        }
    }

    let device_depot = DeviceDepot::new(devices);

    let mut rocket = rocket::build();
    // Add configuration data and other state.
    rocket = rocket.manage(config);
    rocket = rocket.manage(device_depot);
    // Mount "server info" root endpoint.
    rocket = rocket.mount("/", routes![index]);
    // Mount other endpoints.
    rocket = api_auth::register(rocket);
    rocket = api_device::register(rocket);
    // Start server.
    rocket.launch().await
}

#[get("/")]
fn index(config: &State<HubConfig>) -> Json<HubInfo> {
    Json::from(HubInfo {
        name: config.name.clone(),
        description: config.description.clone(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// A basic data object which describes the properties of this hub server.
#[derive(Debug, Clone, Serialize)]
struct HubInfo {
    version: String,
    name: String,
    description: Option<String>,
}

/// A generic error object that can be serialized into JSON.
#[derive(Debug, Clone, Serialize)]
struct ErrorMessage {
    message: String,
}

impl<'a> From<&str> for ErrorMessage {
    fn from(value: &str) -> Self {
        ErrorMessage {
            message: value.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(message: String) -> Self {
        ErrorMessage { message }
    }
}
