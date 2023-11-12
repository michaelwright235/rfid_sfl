#[cfg(test)]
use crate::{devices::Device, rfid_items::DanishRfidItem, routes::write_tags::WriteResponse};

#[cfg(test)]
pub struct TestDevice;

#[cfg(test)]
impl Device for TestDevice {
    fn connect(&mut self) {}

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
        if item.set_item_id("1234567890").is_err() {
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
