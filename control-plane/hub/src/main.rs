#[macro_use]
extern crate rocket;

use crate::config::HubConfig;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use std::process::exit;

/// Module where all authentication code resides. In particular, it defines `/login` and `/renew`
/// endpoints. It also provides `ApiToken` request guard that can be used to enforce authentication
/// on other endpoints.
mod api_auth;

/// Describes configuration of the hub server, including deserialization from a JSON file.
mod config;

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

    let mut rocket = rocket::build();
    // Add configuration data.
    rocket = rocket.manage(config);
    // Mount "server info" root endpoint.
    rocket = rocket.mount("/", routes![index]);
    // Mount authentication related endpoints.
    rocket = api_auth::register(rocket);
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
