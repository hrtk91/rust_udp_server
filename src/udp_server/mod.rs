use std::thread::JoinHandle;
use std::net::UdpSocket;
use std::result::Result;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver, TryRecvError };
use std::str;

#[derive(Debug)]
pub struct UdpServer {
    pub host: String,
    pub quit_code: String,
    handle: JoinHandle<()>,
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl UdpServer {
    pub fn try_recv(&self) -> Result<String, TryRecvError> {
        self.rx.try_recv()
    }

    pub fn close(self) -> () {
        println!("closing...");
        self.tx.send(self.quit_code).unwrap();
        self.handle.join().unwrap();
        println!("closed");
    }
}

pub fn listen(host: &str) -> Result<UdpServer, std::io::Error> {
    let socket = UdpSocket::bind(host).expect("failed bind");
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (tx2, rx2): (Sender<String>, Receiver<String>) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let mut buff = [0; 1024];
        socket.set_nonblocking(true).unwrap();
        loop {
            let body = match rx.try_recv() {
                Ok(body) => body,
                Err(e) => match e {
                    TryRecvError::Empty => "".to_string(),
                    TryRecvError::Disconnected => ":q".to_string(),
                }
            };

            if body == ":q" {
                return;
            }
            
            #[allow(dead_code)]
            if let Ok((buff_size, socket_addr)) = socket.recv_from(&mut buff) {
                log::info!("buff_size: {} socket_addr: {:?}", buff_size, socket_addr);
                if buff_size != 0 {
                    let body = str::from_utf8(&buff[..buff_size]).unwrap();
                    tx2.send(body.to_string())
                        .expect("受信したデータの展開に失敗");
                    log::trace!("body:{}", body.escape_debug());
                    buff = [0; 1024];
                }
            }
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
