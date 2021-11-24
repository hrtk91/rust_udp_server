#[path = "model/model.rs"]
mod model;
#[path = "udp_server/udp_server.rs"]
mod udp_server;
#[path = "mvc/mvc.rs"]
mod mvc;
mod controller;

use std::any::Any;
use controller::RoomController;
use model::room_manager::RoomManager;
use mvc::{ Mvc, Scope };
use mvc::request::{ Request };
use mvc::middleware::{ PathFilter };
use mvc::util::AsAny;
use mvc::controller::Controller;

#[macro_export]
macro_rules! nameof {
    ($ty: ty) => {
        stringify!($ty)
    };
}

fn init_mvc() -> Mvc {
    let mut mvc = Mvc::new();
    mvc.add_transient(nameof!(RoomController), || RoomController::new().as_any())
       .add_singleton(nameof!(RoomManager), Box::new(RoomManager::new()))
       .add_singleton(nameof!(PathFilter), PathFilter::new().as_any());
    mvc
}

fn main() {
    env_logger::init();

    let udp_server = udp_server::listen("0.0.0.0:8080").unwrap();
    let mut mvc = init_mvc();

    loop {
        let udp_request = udp_server.try_recv();

        match udp_request.body.trim_end() {
            v if v == udp_server.quit_code.as_str() => break,
            "" => continue,
            _ => (),
        };

        let mut request: Request = match serde_json::from_str(&udp_request.body) {
            Ok(json) => json,
            Err(_) => {
                udp_server.bad_request(udp_request.src_addr);
                continue;
            },
        };

        log::trace!("request: {:?}", request);

        if let Err(e) = mvc.invoke_middlewares(&mut request) {
            log::error!("{}", e);
            udp_server.error(udp_request.src_addr);
        }

        log::trace!("request: {:?}", request);

        let controller_name = match request.header.get("controller_name_without_prefix") {
            Some(string) => string,
            None => "",
        };

        if let Some(func) = mvc.get(&format!("{}Controller", controller_name), Scope::Transient) {
            let func = func.downcast_ref::<Box<dyn Fn() -> Box<dyn Any>>>().unwrap();
            let mut controller = func().downcast::<Box<dyn Controller>>().unwrap();
            let resp = match controller.invoke(&mut request, &mut mvc) {
                Ok(resp) => match serde_json::to_string(&resp) {
                    Ok(json) => json,
                    Err(e) => {
                        log::error!("{:?}", e);
                        udp_server.error(udp_request.src_addr);
                        continue;
                    },
                },
                Err(e) => {
                    log::error!("{:?}", e);
                    udp_server.error(udp_request.src_addr);
                    continue;
                },
            };

            udp_server.send_or_error(resp, udp_request.src_addr);
        };
    }

    udp_server.close();
}
