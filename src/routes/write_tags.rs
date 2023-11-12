use log::*;
use rocket::State;
use rocket::form::Form;
use rocket_client_addr::ClientAddr;
use crate::{devices::DevicesList, rfid_items::DanishRfidItem, config::Config};
use rocket::serde::{Serialize, Deserialize, json};

use super::{check_if_addr_local, RfidResponse, RfidStatusResponse};

#[derive(FromForm, Debug)]
#[allow(non_snake_case)]
pub struct WriteRequest<'r> {
    action: &'r str,
    deviceId: &'r str,
    id: Vec<&'r str>,
    itemId: Vec<&'r str>,
    r#type: Vec<u8>,
    libraryId: Vec<&'r str>,
    itemSize: Vec<u8>,
    indexInItemPack: Vec<u8>,
    // isSecuritySupported: Vec<bool>,
    // expirationDate: Vec<&'r str>,
    // isSecured: Vec<bool>,
    // circulationType: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct WriteResponse {
    pub id: String,
    pub success: bool,
    pub error: Option<WriteError>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct WriteError {
    pub r#type: String,
    pub message: String,
}

// POST http://127.0.0.1:21646/rfid/
#[post("/", data="<params>")]
pub fn handler(
    shared_resource: &State<DevicesList>, 
    config: &State<Config>,
    client_addr: &ClientAddr, 
    params: Form<WriteRequest<'_>>) -> RfidStatusResponse {
    // Check if remote address is local. If it's not then exit
    if let Err(r) = check_if_addr_local(client_addr) {
        return r;
    }

    if params.action != "writeTags" {
        return RfidStatusResponse::Err404( RfidResponse::default() );
    }

    let get_device = shared_resource.inner().get().get(&params.deviceId.to_string());
    if get_device.is_none() {
        return RfidStatusResponse::Err404( RfidResponse::default() );
    }

    let device_mutex = get_device.unwrap();
    let mut device = device_mutex.lock().unwrap();
    device.connect();

    debug!("Write tag: {:?}", params);

    // Check if params are valid
    if params.itemId.len() != params.id.len() ||
       params.itemId.len() != params.r#type.len() ||
       params.itemId.len() != params.libraryId.len() ||
       params.itemId.len() != params.itemSize.len() ||
       params.itemId.len() != params.indexInItemPack.len()
    {
        debug!("Params are not valid!");
        return RfidStatusResponse::Err400( RfidResponse::default() );
    }

    for lib in &params.libraryId {
        if lib.len() < 3 {
            debug!("Library id is not valid!");
            return RfidStatusResponse::Err400( RfidResponse::default() );
        }
    }

    let mut items = Vec::with_capacity(params.itemId.len());

    for i in 0..params.itemId.len() {
        let mut item = DanishRfidItem::default();
        item.set_number_of_parts(params.itemSize[i]);
        item.set_ordinal_number(params.indexInItemPack[i]);
        if 
           item.set_item_id(params.itemId[i]).is_err() ||
           item.set_usage_type(params.r#type[i]).is_err() ||
           item.set_country(&params.libraryId[i][0..2]).is_err() ||
           item.set_library_id(&params.libraryId[i][3..]).is_err()
        {
            debug!("Params of an item are not valid!");
            return RfidStatusResponse::Err400( RfidResponse::default() );
        }
        debug!("item = {:?}", item);
        items.push(item);
    }

    let mut confirm = true;

    // Shows confirm dialog to a user
    if config.ask_when_writing() {
        confirm = rocket::tokio::task::block_in_place(|| {
            native_dialog::MessageDialog::new()
            .set_title("RFID Server For Libraries")
            .set_text("Записать карту на считывателе?")
            .set_type(native_dialog::MessageType::Info).show_confirm().unwrap_or(true)
        });
        info!("User confirmation for writing a card: {confirm}");
    }

    if confirm {
        let responses = device.write_tags(items);
        debug!("Write tag responses: {:?}",responses);
        info!("Card(s) has been successfully written");
        RfidStatusResponse::Ok(
            RfidResponse::from_string(json::to_string(&responses).unwrap())
        )
    } else {
        RfidStatusResponse::Err404( RfidResponse::default() )
    }

}

// OPTIONS http://127.0.0.1:21646/rfid/
#[options("/")]
pub fn handler_options(client_addr: &ClientAddr) -> RfidStatusResponse {
    // Check if remote address is local. If it's not then exit
    if let Err(r) = check_if_addr_local(client_addr) {
        return r;
    }
    RfidStatusResponse::Ok( RfidResponse::default() )
}
