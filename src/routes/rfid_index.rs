use rocket_client_addr::ClientAddr;
use super::{check_if_addr_local, RfidResponse, RfidStatusResponse};

// GET http://127.0.0.1:21646/rfid/
#[get("/")]
pub fn handler(client_addr: &ClientAddr) -> RfidStatusResponse {
    // Check if remote address is local. If not then exit
    if let Err(r) = check_if_addr_local(client_addr) {
        return r;
    }
    let mut response = RfidResponse::from_str("./rfid works");
    response.make_html();
    RfidStatusResponse::Ok(response)
}
