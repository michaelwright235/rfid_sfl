use log::*;
use rocket::serde::{Serialize, Deserialize, json};
use rocket::State;
use rocket_client_addr::ClientAddr;
use crate::devices::DevicesList;
use super::{check_if_addr_local, RfidResponse, RfidStatusResponse};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ItemResponse {
    id: Option<String>,
    r#type: u8,
    tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct Tag {
    tagId: String,
    itemId: Option<String>,
    format: i16,
    r#type: u8,
    itemSize: u8,
    indexInItemPack: u8,
    libraryId: String,
}

// GET http://127.0.0.1:21646/rfid/?action=getItemsList&deviceId=<deviceId>
#[allow(non_snake_case)]
#[get("/?action=getItemsList&<deviceId>")]
pub fn handler(
    shared_resource: &State<DevicesList>,
    client_addr: &ClientAddr, deviceId: &str
) -> RfidStatusResponse {
    // Check if remote address is local. If not then exit
    if let Err(r) = check_if_addr_local(&client_addr) {
        return r;
    }

    let get_device = shared_resource.inner().get().get(&deviceId.to_string());
    if get_device.is_none() {
        debug!("Wrong device");
        return RfidStatusResponse::Err404( RfidResponse::default() );
    }

    let device_mutex = get_device.unwrap();
    let mut device = device_mutex.lock().unwrap();
    device.connect();

    if !device.is_connected() {
        return RfidStatusResponse::Err404(RfidResponse::from_str(""));
    }

    let items = device.get_items();
    if items.len() == 0 {
        info!("No cards found");
        return RfidStatusResponse::Ok(
            RfidResponse::from_str("[]")
        );
    }

    let mut item_responses = Vec::with_capacity(items.len());
    for item in items {
        if item.is_empty() {
            item_responses.push(ItemResponse {
                id: None,
                r#type: 0,
                tags: vec![Tag {
                    tagId: "{}".to_string(),
                    itemId: None,
                    format: -1,
                    r#type: 0,
                    itemSize: 0,
                    indexInItemPack: 0,
                    libraryId: "".to_string() }]
            });
            continue;
        }
        let mut library_id = item.country().to_owned();
        library_id.push('-');
        library_id.push_str(item.library_id());
        
        let tag = Tag {
            tagId: item.card_id_string(),
            itemId: Some(item.item_id().to_owned()),
            format: 61,
            r#type: item.usage_type(),
            itemSize: item.number_of_parts(),
            indexInItemPack: item.ordinal_number(),
            libraryId: library_id
        };
        item_responses.push(ItemResponse {
            id: tag.itemId.to_owned(),
            r#type: tag.r#type.to_owned(),
            tags: vec![tag]
        });
    }
    
    
    let response = json::to_string(&item_responses).unwrap();
    debug!("{response}");

    RfidStatusResponse::Ok(
        RfidResponse::from_string(response)
    )
}

// OPTIONS http://127.0.0.1:21646/rfid/?action=getItemsList&deviceId=<deviceId>
#[options("/?action=getItemsList&<deviceId>")]
#[allow(non_snake_case)]
pub fn handler_options(
    shared_resource: &State<DevicesList>,
    client_addr: &ClientAddr, deviceId: &str
) -> RfidStatusResponse {
    handler(shared_resource, client_addr, deviceId)
}
