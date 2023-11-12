use super::Device;
use crate::devices::WriteResponse;
use crate::rfid_items::DanishRfidItem;
use crate::routes::write_tags::WriteError;
use bibliotheca_rfid_reader::*;

const VID: u16 = 0x0d2c;
const PID: u16 = 0x032a;

pub struct Bibliotheca {
    handle: Result<BibliothecaRfidReader, ReaderError>,
}

impl Bibliotheca {
    pub fn new() -> Self {
        #[cfg(unix)]
        let _ = BibliothecaRfidReader::set_vid_pid(VID, PID);

        Self {
            handle: BibliothecaRfidReader::open(),
        }
    }
}

impl Device for Bibliotheca {
    fn connect(&mut self) {
        // Reopen device if there was an error
        if let Ok(device) = &mut self.handle {
            if device.ftdi_device_info().is_err() {
                self.handle = BibliothecaRfidReader::open();
            }
        } else {
            self.handle = BibliothecaRfidReader::open();
        }
    }

    fn is_connected(&self) -> bool {
        match self.handle {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn multi_tag_is_supported(&self) -> bool {
        true
    }

    fn compound_data_is_supported(&self) -> bool {
        true
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn get_items(&mut self) -> Vec<DanishRfidItem> {
        if let Ok(device) = &mut self.handle {
            if let Ok(inventory) = device.inventory() {
                let mut danish_items = Vec::with_capacity(inventory.len());
                for card in inventory {
                    let bytes = device.read_card(&card).unwrap_or(vec![]);
                    for b in &bytes {
                        print!("{b:#X} ");
                    }
                    println!();
                    match DanishRfidItem::from_bytes(&bytes) {
                        Ok(mut item) => {
                            item.set_card_id(card.to_vec());
                            danish_items.push(item);
                        }
                        Err(_) => (),
                    }
                }
                return danish_items;
            }
        }
        return vec![];
    }

    fn write_tags(&mut self, items: Vec<DanishRfidItem>) -> Vec<WriteResponse> {
        if let Ok(device) = &mut self.handle {
            if items.len() != 1 {
                let mut resps = Vec::with_capacity(items.len());
                for item in items {
                    resps.push(WriteResponse {
                        id: item.card_id_string(),
                        success: false,
                        error: Some(WriteError {
                            r#type: "Write Error".to_string(),
                            message: "Reader can write only one tag at a time".to_string(),
                        }),
                    });
                }
                return resps;
            }
            let item = &items[0];
            if let Ok(()) = device.write_card(item.card_id(), &item.to_bytes()) {
                return vec![WriteResponse {
                    id: item.card_id_string(),
                    success: true,
                    error: None,
                }];
            } else {
                return vec![WriteResponse {
                    id: item.card_id_string(),
                    success: false,
                    error: Some(WriteError {
                        r#type: "Write Error".to_string(),
                        message: "Error during writing a card. Probably there's no cards nearby."
                            .to_string(),
                    }),
                }];
            }
        }
        vec![WriteResponse {
            id: items[0].card_id_string(),
            success: false,
            error: Some(WriteError {
                r#type: "Write Error".to_string(),
                message: "Couldn't connect to the reader".to_string(),
            }),
        }]
    }
}
