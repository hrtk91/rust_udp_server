use std::net::{ SocketAddr };

pub struct UdpResponse {
    pub body: String,
    pub dst_addr: SocketAddr,
}
