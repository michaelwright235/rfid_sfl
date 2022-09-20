use log::*;
use rocket::serde::{Serialize, Deserialize, json};
use rocket::State;
use rocket_client_addr::ClientAddr;
use crate::devices::DevicesList;
use super::{check_if_addr_local, RfidResponse, RfidStatusResponse};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct DeviceJson {
    id: String,
    title: String,
    isOnline: bool,
    manualConnectIsNeeded: bool,
    multiTagIsSupported: bool,
    isError: bool,
    isReadOnly: bool,
    compoundDataIsSupported: bool,
}

// GET http://127.0.0.1:21646/rfid/?action=getDevicesList
#[get("/?action=getDevicesList")]
pub fn handler(shared_resource: &State<DevicesList>, client_addr: &ClientAddr) -> RfidStatusResponse {
    // Check if remote address is local. If not then exit
    if let Err(r) = check_if_addr_local(&client_addr) {
        return r;
    }

    let devices = shared_resource.inner().get();
    let mut devices_json = Vec::with_capacity(devices.len());
    for (name, device) in devices {
        if device.is_connected() {
            info!("Available device: {name}");
            devices_json.push(DeviceJson {
                id: name.to_string(),
                title: device.get_name().to_string(),
                isOnline: device.is_connected(),
                manualConnectIsNeeded: false,
                multiTagIsSupported: device.multi_tag_is_supported(),
                isError: false,
                isReadOnly: device.is_read_only(),
                compoundDataIsSupported: device.compound_data_is_supported(),
            });
        }
    }

    let response = json::to_string(&devices_json).unwrap_or("[]".to_string());
    debug!("{:?}", response);
    RfidStatusResponse::Ok (
        RfidResponse::from_string(response)
    )
}

// OPTIONS http://127.0.0.1:21646/rfid/?action=getDevicesList
#[options("/?action=getDevicesList")]
pub fn handler_options(shared_resource: &State<DevicesList>, client_addr: &ClientAddr) -> RfidStatusResponse {
    handler(shared_resource, client_addr)
}
