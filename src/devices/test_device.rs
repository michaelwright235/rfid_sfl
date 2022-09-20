use crate::{devices::{Device}, rfid_items::DanishRfidItem, routes::write_tags::WriteResponse};

#[derive(Debug)]
pub struct TestDevice;

impl Device for TestDevice {
    fn get_name(&self) -> &str {
        "Test Device"
    }

    fn is_busy(&self) -> bool {
        false
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

    fn is_connected(&self) -> bool {
        true
    }

    fn get_items(&self) -> Vec<DanishRfidItem> {
        let mut item = DanishRfidItem::default();
        if item.set_item_id("RU").is_err() {
            return vec![];
        }
        if item.set_library_id("123").is_err() {
            return vec![];
        }
        if item.set_country("RU").is_err() {
            return vec![];
        }

        vec![item]
    }

    fn write_tags(&self, _: Vec<DanishRfidItem>) -> Vec<WriteResponse> {
        vec![]
    }
}
