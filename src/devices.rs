pub mod test_device;
mod cf_rh320u_93;

use std::collections::HashMap;
use crate::{rfid_items::DanishRfidItem, routes::write_tags::WriteResponse};
use self::{test_device::TestDevice, cf_rh320u_93::CfRh320u93};

pub trait Device: Send + Sync {
    fn get_name(&self) -> &str;
    fn is_busy(&self) -> bool;
    fn is_connected(&self) -> bool;
    fn multi_tag_is_supported(&self) -> bool;
    fn compound_data_is_supported(&self) -> bool;
    fn is_read_only(&self) -> bool;
    fn get_items(&self) -> Vec<DanishRfidItem>; // List<RfidItem>
    //fn set_tags_security(&self, params: HashMap<String, bool>) -> Result<(),()>;
    fn write_tags(&self, items: Vec<DanishRfidItem>) -> Vec<WriteResponse>;
}

pub struct DevicesList {
    devices: HashMap<String, Box<dyn Device>>
}

impl DevicesList {
    pub fn new() -> Self {
        let mut available_devices: Vec<Box<dyn Device>> = Vec::new();
        if cfg!(test) {
            available_devices.push(Box::new(TestDevice{}));
        }
        available_devices.push(Box::new(CfRh320u93{}));

        let mut devices: HashMap<String, Box<dyn Device>> = HashMap::new();

        for kind in available_devices {
            let name = kind.as_ref().get_name();
            devices.insert(name.to_string(),kind);
        }      
        Self { devices: devices }
    }

    pub fn get(&self) -> &HashMap<String, Box<dyn Device>> {
        &self.devices
    }
}
