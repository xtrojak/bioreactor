use crate::device::DeviceStatus;
use crate::{BioreactorDevice, DeviceDepot, ErrorMessage};
use rocket::http::Status;
use rocket::response::status::{Custom, NotFound};
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use serde::{Deserialize, Serialize};
use crate::api_auth::ApiToken;

pub fn register(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![device_list, device_info, device_update])
}

#[derive(Debug, Clone, Serialize)]
struct DeviceList {
    devices: Vec<DeviceSummary>,
}

#[derive(Debug, Clone, Serialize)]
struct DeviceSummary {
    id: String,
    name: String,
    status: DeviceStatus,
}

#[derive(Debug, Clone, Serialize)]
struct DeviceInfo {
    id: String,
    name: String,
    description: Option<String>,
    current_status: DeviceStatus,
    target_status: DeviceStatus,
}

impl DeviceInfo {
    pub fn from_device(device: &BioreactorDevice) -> DeviceInfo {
        DeviceInfo {
            id: device.get_id(),
            name: device.get_name(),
            description: device.get_description(),
            current_status: device.get_current_status(),
            target_status: device.get_target_status(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DeviceInfoUpdate {
    target_status: DeviceStatus,
}

#[get("/hub/<device_id>")]
async fn device_info(
    device_id: &str,
    depot: &State<DeviceDepot>,
    _token: ApiToken
) -> Result<Json<DeviceInfo>, NotFound<Json<ErrorMessage>>> {
    if let Some(device) = depot.get_device(device_id).await {
        Ok(Json::from(DeviceInfo::from_device(&device)))
    } else {
        let message: ErrorMessage = format!("Device with ID `{}` not found.", device_id).into();
        Err(NotFound(Json::from(message)))
    }
}

#[post("/hub/<device_id>", format = "json", data = "<update>")]
async fn device_update(
    device_id: &str,
    update: Json<DeviceInfoUpdate>,
    depot: &State<DeviceDepot>,
    _token: ApiToken
) -> Result<Json<DeviceInfo>, Custom<Json<ErrorMessage>>> {
    if let Some(mut device) = depot.get_device_mut(device_id).await {
        let update = update.into_inner();
        match device.set_target_status(update.target_status) {
            Ok(()) => Ok(Json::from(DeviceInfo::from_device(&device))),
            Err(e) => {
                let message: ErrorMessage = e.into();
                Err(Custom(Status::BadRequest, Json::from(message)))
            }
        }
    } else {
        let message: ErrorMessage = format!("Device with ID `{}` not found.", device_id).into();
        Err(Custom(Status::NotFound, Json::from(message)))
    }
}

#[get("/hub")]
async fn device_list(depot: &State<DeviceDepot>, _token: ApiToken) -> Json<DeviceList> {
    let mut devices = Vec::new();
    for id in depot.device_ids().iter() {
        let device = depot.get_device(id).await.unwrap();
        devices.push(DeviceSummary {
            id: device.get_id(),
            name: device.get_name(),
            status: device.get_current_status(),
        });
    }

    Json::from(DeviceList { devices })
}
