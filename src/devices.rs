mod bibliotheca;
mod cf_rh320u_93;
#[cfg(test)]
mod test_device;

use self::{bibliotheca::Bibliotheca, cf_rh320u_93::CfRh320u93};
use crate::{rfid_items::DanishRfidItem, routes::write_tags::WriteResponse};
use std::{collections::HashMap, sync::Mutex};

type DeviceMutexBoxT = Mutex<Box<dyn Device>>;

pub trait Device: Send + Sync {
    fn connect(&mut self);
    fn is_connected(&self) -> bool;
    fn multi_tag_is_supported(&self) -> bool;
    fn compound_data_is_supported(&self) -> bool;
    fn is_read_only(&self) -> bool;
    fn get_items(&mut self) -> Vec<DanishRfidItem>; // List<RfidItem>
    fn write_tags(&mut self, items: Vec<DanishRfidItem>) -> Vec<WriteResponse>;

    //fn set_tags_security(&self, params: HashMap<String, bool>) -> Result<(),()>;
}

pub struct DevicesList {
    devices: HashMap<String, DeviceMutexBoxT>,
}

impl DevicesList {
    pub fn new() -> Self {
        let mut devices: HashMap<String, DeviceMutexBoxT> = HashMap::new();

        #[cfg(test)]
        devices.insert(
            "Test Device".to_string(),
            Mutex::new(Box::new(self::test_device::TestDevice {})),
        );
        devices.insert(
            "Chafon CF-RH320U-93".to_string(),
            Mutex::new(Box::new(CfRh320u93::new())),
        );
        devices.insert(
            "Bibliotheca 210 Reader".to_string(),
            Mutex::new(Box::new(Bibliotheca::new())),
        );

        Self { devices }
    }

    pub fn get(&self) -> &HashMap<String, DeviceMutexBoxT> {
        &self.devices
    }
}
