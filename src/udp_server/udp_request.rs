use std::net::SocketAddr;

pub struct UdpRequest {
    pub body: String,
    pub requester_ip: SocketAddr,
}
