#[path = "model/model.rs"]
mod model;
#[path = "udp_server/udp_server.rs"]
mod udp_server;
use model::room_manager::RoomManager;
use model::request::{ Request };

fn main() {
    env_logger::init();

    let udp_server = udp_server::listen("0.0.0.0:8080").unwrap();
    let mut room_manager = RoomManager::new();

    loop {
        let udp_request = udp_server.try_recv();

        match udp_request.body.trim_end() {
            v if v == udp_server.quit_code.as_str() => break,
            "" => continue,
            _ => (),
        };

        let request: Request = match serde_json::from_str(&udp_request.body) {
            Ok(json) => json,
            Err(_) => serde_json::from_str("{}").unwrap(),
        };

        log::trace!("request: {:?}", request);

        match request.req_type.unwrap_or_default().as_str() {
            "create_room" => {
                match room_manager.create_room(request.payload) {
                    Ok(json) => if let Err(_) = udp_server.try_send(json, udp_request.src_addr) {
                        udp_server.error(udp_request.src_addr);
                    },
                    Err(_) => udp_server.error(udp_request.src_addr),
                };
            },
            "add_user" => {
                if let Err(e) = room_manager.add_user(request.payload) {
                    log::trace!("ユーザー追加に失敗。：{}", e);
                }
            },
            "update_user" => {
                if let Err(e) = room_manager.update_user(request.payload) {
                    log::trace!("ユーザー更新に失敗。：{}", e);
                }
            },
            "remove_user" => {
                match room_manager.remove_user(request.payload) {
                    Ok(_) => udp_server.ok(udp_request.src_addr),
                    Err(_) => udp_server.error(udp_request.src_addr),
                };
            },
            _ => (),
        };
        
    }

    udp_server.close();
}
