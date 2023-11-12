use log::*;

#[derive(Debug)]
pub struct DanishRfidItem {
    empty: bool,
    card_id: Vec<u8>,
    usage_type: u8, // u4 in fact
    standart_version: u8, // u4 in fact
    number_of_parts: u8,
    ordinal_number: u8,
    item_id: String, // max 16 chars long
    country: String, // max 2 chars long
    library_id: String // max 11 chars long
}

impl Default for DanishRfidItem {
    fn default() -> Self {
        Self {
            empty: false,
            card_id: vec![],
            usage_type: 8,
            standart_version: 1,
            number_of_parts: 1,
            ordinal_number: 1,
            item_id: Default::default(),
            country: Default::default(),
            library_id: Default::default()
        }
    }
}

#[allow(dead_code)]
impl DanishRfidItem {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() < 23 {
            return Err(());
        }

        // Checking checksum
        let crc = bytes[19] as u16 | ((bytes[20] as u16) << 8);
        let mut bytes_without_crc = Vec::from(bytes);
        bytes_without_crc.remove(19);
        bytes_without_crc.remove(19);
        bytes_without_crc.push(0);
        bytes_without_crc.push(0);
        let true_crc = Self::calc_crc(&bytes_without_crc);
        if crc != true_crc {
            info!("Wrong checksum! Returning empty card");
            return Ok(Self {
                empty: true,
                ..Default::default()
            });
        }

        let usage_type = bytes[0] >> 4;
        let standart_version = bytes[0] & 0x0F;
        let number_of_parts = bytes[1];
        let ordinal_number = bytes[2];
        

        let item_id = match String::from_utf8(Self::strip0s(&bytes[3..19]).to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(())
        };
        
        let country = match String::from_utf8(Self::strip0s(&bytes[21..23]).to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(())
        };
        
        let library_id = match String::from_utf8(Self::strip0s(&bytes[23..]).to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(())
        };
        
        Ok(Self {
            empty: false,
            card_id: vec![],
            usage_type,
            standart_version,
            number_of_parts,
            ordinal_number,
            item_id,
            country,
            library_id
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32);
        bytes.push((self.usage_type << 4) + self.standart_version);
        bytes.push(self.number_of_parts);
        bytes.push(self.ordinal_number);
        for b in self.item_id.as_bytes() {
            bytes.push(*b);
        }
        for _ in self.item_id.len()..16 {
            bytes.push(0x0);
        }
        for b in self.country.as_bytes() {
            bytes.push(*b);
        }
        for b in self.library_id.as_bytes() {
            bytes.push(*b);
        }
        for _ in self.library_id.len()..11 {
            bytes.push(0x0);
        }

        let crc = Self::calc_crc(&bytes).to_be_bytes();
        bytes.insert(19, crc[1]);
        bytes.insert(20, crc[0]);

        bytes
    }

    fn strip0s(bytes: &[u8]) -> &[u8] {
        let mut new_len = bytes.len();
        for i in 0..(bytes.len()-1) {
            if bytes[bytes.len()-1-i] == 0x00 {
                new_len = bytes.len()-1-i;
            } else {
                break;
            }
        }
        &bytes[0..new_len]
    }

    pub fn calc_crc(bytes: &[u8]) -> u16 {
        let crc_poly = 0x1021;
        let mut crc_sum: u16 = 0xffff;

        for b in bytes {
            let mut c = *b as u16;
            c<<=8;
            for _ in 0..8 {
                let xor_flag=((crc_sum ^ c) & 0x8000)!=0;
                crc_sum <<= 1;
                if xor_flag {
                    crc_sum ^= crc_poly;
                }
                c <<= 1;
            }
            crc_sum &= 0xffff;
        }
        crc_sum
    }

    pub fn set_usage_type(&mut self, usage_type: u8) -> Result<(), ()> {
        if usage_type > 15 {
            return Err(());
        }
        self.usage_type = usage_type;
        Ok(())
    }
    pub fn usage_type(&self) -> u8 {
        self.usage_type
    }

    pub fn set_standart_version(&mut self, standart_version: u8) -> Result<(), ()> {
        if standart_version > 15 {
            return Err(());
        }
        self.standart_version = standart_version;
        Ok(())
    }
    pub fn standart_version(&self) -> u8 {
        self.standart_version
    }

    pub fn set_number_of_parts(&mut self, number_of_parts: u8) {
        self.number_of_parts = number_of_parts;
    }
    pub fn number_of_parts(&self) -> u8 {
        self.number_of_parts
    }

    pub fn set_ordinal_number(&mut self, ordinal_number: u8) {
        self.ordinal_number = ordinal_number;
    }
    pub fn ordinal_number(&self) -> u8 {
        self.ordinal_number
    }

    pub fn set_item_id(&mut self, item_id: &str) -> Result<(), ()> {
        if item_id.bytes().len() > 16 {
            return Err(());
        }
        self.item_id = item_id.to_string();
        Ok(())
    }
    pub fn item_id(&self) -> &String {
        &self.item_id
    }

    pub fn set_country(&mut self, country: &str) -> Result<(), ()> {
        if country.bytes().len() > 2 {
            return Err(());
        }
        self.country = country.to_string();
        Ok(())
    }
    pub fn country(&self) -> &String {
        &self.country
    }

    pub fn set_library_id(&mut self, library_id: &str) -> Result<(), ()> {
        if library_id.bytes().len() > 11 {
            return Err(());
        }
        self.library_id = library_id.to_string();
        Ok(())
    }
    pub fn library_id(&self) -> &String {
        &self.library_id
    }

    pub fn set_card_id(&mut self, card_id: Vec<u8>) {
        self.card_id = card_id;
    }
    pub fn set_card_id_string(&mut self, card_id: &str) -> Result<(), ()>{
        fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
            let mut subs = Vec::with_capacity(string.len() / sub_len);
            let mut iter = string.chars();
            let mut pos = 0;
        
            while pos < string.len() {
                let mut len = 0;
                for ch in iter.by_ref().take(sub_len) {
                    len += ch.len_utf8();
                }
                subs.push(&string[pos..pos + len]);
                pos += len;
            }
            subs
        }
        let sub_str = sub_strings(card_id, 2);
        let mut values = Vec::with_capacity(sub_str.len());
        for v in sub_str {
            if let Ok(i) = u8::from_str_radix(v, 16) {
                values.push(i);
            } else {
                return Err(());
            }
        }
        self.card_id = values;
        Ok(())
    }
    pub fn card_id(&self) -> &[u8] {
        &self.card_id
    }
    pub fn card_id_string(&self) -> String {
        let mut s = String::new();
        for b in &self.card_id {
            s.push_str( format!("{:02X}", b).as_str() );
        }
        s
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

}

#[test]
fn bytes_to_item() {
    let bytes = [0x81, 0x01, 0x01, 0x32, 0x39, 0x33, 0x35, 0x30, 0x30, 0x30, 0x30, 0x30, 0x33, 0x36, 0x34, 0x39, 0x00, 0x00, 0x00, 0x87, 0x93, 0x52, 0x55, 0x32, 0x39, 0x33, 0x00, 0x00, 0x00,];
    let item = DanishRfidItem::from_bytes(&bytes).unwrap();
    println!("{:?}", item);
}

#[test]
fn item_to_bytes() {
    let item = DanishRfidItem {
        empty: false,
        card_id: vec![],
        usage_type: 8,
        standart_version: 1,
        number_of_parts: 1,
        ordinal_number: 1,
        item_id: "2935000003649".to_string(),
        country: "RU".to_string(),
        library_id: "293".to_string()
    };
    for b in item.to_bytes() {
        print!("{:#X} ", b);
    }
    println!();
}

#[test]
fn crc() {
    // should be 0x87 0x93
    let bytes_without_crc = [0x81, 0x01, 0x01, 0x32, 0x39, 0x33, 0x35, 0x30, 0x30, 0x30, 0x30, 0x30, 0x33, 0x36, 0x34, 0x39, 0x00, 0x00, 0x00, 0x52, 0x55, 0x32, 0x39, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    // should be 0x1A 0xEE
    //let string = "RFID tag data model".as_bytes();
    println!("{:#X}", DanishRfidItem::calc_crc(&bytes_without_crc));
}
