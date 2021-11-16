use crate::udp_server::udp_request::UdpRequest;
use crate::udp_server::udp_response::UdpResponse;

pub struct UdpThreadMessage {
    pub request: Option<UdpRequest>,
    pub response: Option<UdpResponse>,
}
