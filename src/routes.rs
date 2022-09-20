pub mod index;
pub mod rfid_index;
pub mod get_devices_list;
pub mod get_items_list;
pub mod write_tags;

use log::*;
use rocket::http::{Header, ContentType};
use rocket_client_addr::ClientAddr;

// Wrapper for a response with a http status
#[derive(Responder)]
pub enum RfidStatusResponse {
    #[response(status = 200)]
    Ok(RfidResponse),
    #[response(status = 400)]
    Err400(RfidResponse),
    #[response(status = 403)]
    Err403(RfidResponse),
    #[response(status = 404)]
    Err404(RfidResponse)
}

// Response struct. Inner is the contnets of a response
// The next fields are headers
#[derive(Responder)]
pub struct RfidResponse {
    inner: String,
    header: Header<'static>,
    content_type: ContentType
}

impl Default for RfidResponse {
    fn default() -> Self {
        Self {
            inner: "".to_string(),
            header: Header::new("Access-Control-Allow-Origin", "*"),
            content_type: ContentType::new("application","json;charset=utf-8")
        }
    }
}

impl RfidResponse {
    pub fn from_string(s: String) -> Self {
        Self { inner: s, ..Default::default() }
    }
    pub fn from_str(s: &str) -> Self {
        Self { inner: s.to_string(), ..Default::default() }
    }

    pub fn make_html(&mut self) {
        self.content_type = ContentType::HTML;
    }
}

// Checks if remote address is local
fn check_if_addr_local(client_addr: &ClientAddr) -> Result<(), RfidStatusResponse> {
    //todo ipv6 check
    if let Some(addr) = client_addr.get_ipv4_string() {
        if addr == "127.0.0.1" || addr == "0.0.0.0" || addr == "127.0.0.0" {
            return Ok(());
        }
    }
    warn!("Remote address {} isn't local. Interrupting...",
        client_addr.get_ipv4_string().unwrap_or("{unknown}".to_string())
    );
    Err( RfidStatusResponse::Err403( RfidResponse::default() ) )
}
