pub mod udp_request;
pub mod udp_response;
mod udp_thread_message;
use udp_response::UdpResponse;
use udp_request::UdpRequest;
use std::net::SocketAddr;
use std::thread::JoinHandle;
use std::net::{ UdpSocket, IpAddr, Ipv4Addr };
use std::result::Result;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver, TryRecvError, SendError };
use std::str;
use udp_thread_message::UdpThreadMessage;

#[derive(Debug)]
pub struct UdpServer {
    pub host: String,
    pub quit_code: String,
    handle: JoinHandle<()>,
    tx: Sender<UdpThreadMessage>,
    rx: Receiver<UdpThreadMessage>,
}

impl UdpServer {
    pub fn try_recv(&self) -> UdpRequest {
        match self.rx.try_recv() {
            Ok(msg) => match msg.request {
                Some(req) => req,
                None => UdpRequest::empty(),
            },
            Err(e) => match e {
                TryRecvError::Empty => UdpRequest::empty(),
                TryRecvError::Disconnected => {
                    let mut req = UdpRequest::empty();
                    req.body = self.quit_code.clone();
                    req
                },
            },
        }
    }

    pub fn try_send(&self, response: String, dst_addr: SocketAddr) -> Result<(), SendError<UdpThreadMessage>> {
        self.tx.send(UdpThreadMessage {
            request: None,
            response: Some(UdpResponse {
                body: response,
                dst_addr: dst_addr,
            }),
        })
    }

    pub fn ok(&self, dst_addr: SocketAddr) -> () {
        if let Err(e) = self.try_send("{ \"status\": ok }".to_string(), dst_addr) {
            log::error!("{:?}", e);
        }
    }

    pub fn error(&self, dst_addr: SocketAddr) -> () {
        let body = "{ \"status\":\"internal server error\"}".to_string();
        if let Err(_) = self.try_send(body, dst_addr) {
            log::error!("failed send error");
        }
    }

    pub fn close(self) -> () {
        log::info!("udp_server closing...");
        self.tx.send(UdpThreadMessage {
            request: Some(UdpRequest {
                body: self.quit_code,
                src_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            }),
            response: None,
        }).unwrap();
        self.handle.join().unwrap();
        log::info!("udp_server closed");
    }
}

pub fn listen(host: &str) -> Result<UdpServer, std::io::Error> {
    let socket = UdpSocket::bind(host).expect("failed bind");
    let (tx, rx): (Sender<UdpThreadMessage>, Receiver<UdpThreadMessage>) = mpsc::channel();
    let (tx2, rx2): (Sender<UdpThreadMessage>, Receiver<UdpThreadMessage>) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let mut buff = [0; 1024];
        socket.set_nonblocking(true).unwrap();
        loop {
            let udp_req = match rx.try_recv() {
                Ok(msg) => msg.response,
                Err(e) => match e {
                    TryRecvError::Disconnected => Some(UdpResponse {
                        body: ":q".to_string(),
                        dst_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                    }),
                    TryRecvError::Empty => None,
                }
            };

            if let Some(udp_req) = udp_req {
                if udp_req.body == ":q" {
                    return;
                }
                let body_bytes = udp_req.body.as_bytes();
                match socket.send_to(body_bytes, udp_req.dst_addr) {
                    Ok(_) => log::trace!("send {:?} to {:?}", udp_req.body, udp_req.dst_addr),
                    Err(_) => log::error!("failed send {:?} to {:?}", udp_req.body, udp_req.dst_addr)
                };
            }
            
            if let Ok((size, socket_addr)) = socket.recv_from(&mut buff) {
                log::info!("buff_size: {} socket_addr: {:?}", size, socket_addr);
                if size != 0 {
                    let body = match str::from_utf8(&buff[..size]) {
                        Ok(body) => {
                            log::trace!(
                                "body={}, ip={:?}",
                                body.escape_debug(),
                                socket_addr);
                            body
                        },
                        Err(_) => {
                            log::warn!(
                                "failed received data parse buff={:?} ip={:?}",
                                &buff[..size],
                                socket_addr);
                            ""
                        }
                    };

                    tx2.send(UdpThreadMessage {
                        request: Some(UdpRequest {
                            body: body.to_string(),
                            src_addr: socket_addr,
                        }),
                        response: None,
                    }).expect("受信したデータの展開に失敗");
                }
            }

            buff = [0; 1024];
        }
    });

    Ok(UdpServer {
        host: host.to_string(),
        quit_code: ":q".to_string(),
        handle: handle,
        tx: tx,
        rx: rx2,
    })
}
