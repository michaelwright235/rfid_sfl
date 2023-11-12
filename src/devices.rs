mod cf_rh320u_93;
mod test_device;

use self::cf_rh320u_93::CfRh320u93;
#[cfg(test)]
use self::test_device::TestDevice;
use crate::{rfid_items::DanishRfidItem, routes::write_tags::WriteResponse};
use std::{collections::HashMap, sync::Mutex};

type DeviceMutexBoxT = Mutex<Box<dyn Device>>;

pub trait Device: Send + Sync {
    fn connect(&mut self);
    fn is_connected(&self) -> bool;
    fn multi_tag_is_supported(&self) -> bool;
    fn compound_data_is_supported(&self) -> bool;
    fn is_read_only(&self) -> bool;
    fn get_items(&self) -> Vec<DanishRfidItem>; // List<RfidItem>
                                                //fn set_tags_security(&self, params: HashMap<String, bool>) -> Result<(),()>;
    fn write_tags(&self, items: Vec<DanishRfidItem>) -> Vec<WriteResponse>;
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
            Mutex::new(Box::new(TestDevice {})),
        );
        devices.insert(
            "Chafon CF-RH320U-93".to_string(),
            Mutex::new(Box::new(CfRh320u93::new())),
        );

        Self { devices }
    }

    pub fn get(&self) -> &HashMap<String, DeviceMutexBoxT> {
        &self.devices
    }
}
