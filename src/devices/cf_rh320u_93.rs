use cf_rh320u_93_reader::*;
use crate::rfid_items::DanishRfidItem;
use crate::devices::WriteResponse;
use crate::routes::write_tags::WriteError;
use super::Device;

#[derive(Debug)]
pub struct CfRh320u93;

impl Device for CfRh320u93 {
    fn get_name(&self) -> &str {
        "Chafon CF-RH320U-93"
    }

    fn is_busy(&self) -> bool {
        todo!()
    }

    fn is_connected(&self) -> bool {
        match CFRH320U93::open() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn multi_tag_is_supported(&self) -> bool {
        false
    }

    fn compound_data_is_supported(&self) -> bool {
        true
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn get_items(&self) -> Vec<DanishRfidItem> {
        if let Ok(device) = CFRH320U93::open() {
            let _ = device.control_led(0x01, 0x10);

            if let Ok(inventory) = device.iso15693_inventory() {
                // this reader supports reading only 1 card at a time
                if inventory.len() == 1 {
                    let bytes = device.iso15693_read(AccessFlag::WithoutUID, 0, 0x08).unwrap_or(vec![]);
                    match DanishRfidItem::from_bytes(&bytes) {
                        Ok(mut item) => {
                            item.set_card_id(inventory[0].to_vec());
                            return vec![item];
                        },
                        Err(_) => ()
                    }
                }
            }
        }
        return vec![];
    }

    fn write_tags(&self, items: Vec<DanishRfidItem>) -> Vec<WriteResponse> {
        if let Ok(device) = CFRH320U93::open() {
            let _ = device.control_led(0x01, 0x10);

            if items.len() != 1 {
                let mut resps = Vec::with_capacity(items.len());
                for item in items {
                    resps.push(WriteResponse {
                        id: item.card_id_string(),
                        success: false,
                        error: Some(WriteError{
                            r#type: "Write Error".to_string(),
                            message: "Reader can write only one tag at a time".to_string()
                        })
                    });
                }
                return resps;
            }
            let item = items[0].to_bytes();
            if let Ok(()) = device.iso15693_write(AccessFlag::WithoutUID, 0, &item) {
                return vec![WriteResponse {
                    id: items[0].card_id_string(),
                    success: true,
                    error: None
                }];
            } else {
                return vec![WriteResponse {
                    id: items[0].card_id_string(),
                    success: false,
                    error: Some(WriteError{
                        r#type: "Write Error".to_string(),
                        message: "Error during writing a card. Probably there's no cards nearby.".to_string()
                    })
                }];
            }
        }
        vec![]
    }
}