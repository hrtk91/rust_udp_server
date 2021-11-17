use std::net::{ SocketAddr, IpAddr, Ipv4Addr };

pub struct UdpRequest {
    pub body: String,
    pub src_addr: SocketAddr,
}

impl UdpRequest {
    pub fn empty() -> UdpRequest {
        UdpRequest {
            body: "".to_string(),
            src_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        }
    }
}
