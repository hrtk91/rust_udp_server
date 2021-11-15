#[path = "model/model.rs"]
mod model;
mod udp_server;
use std::sync::mpsc::{ TryRecvError };
use model::room_manager::RoomManager;
use model::request::{ Request };

fn main() {
    env_logger::init();

    let udp_server = udp_server::listen("0.0.0.0:8080").unwrap();
    let mut room_manager = RoomManager::new();

    loop {
        let body = match udp_server.try_recv() {
            Ok(body) => body.trim_end().to_string(),
            Err(e) => match e {
                TryRecvError::Empty => "".to_string(),
                TryRecvError::Disconnected => udp_server.quit_code.clone(),
            }
        };

        match body.as_str() {
            v if v == udp_server.quit_code.as_str() => break,
            "" => continue,
            _ => (),
        };

        let request: Request = match serde_json::from_str(&body) {
            Ok(json) => json,
            Err(_) => serde_json::from_str("{}").unwrap(),
        };

        log::trace!("request: {:?}", request);

        match request.req_type.unwrap_or_default().as_str() {
            "create_room" => {
                if let Err(e) = room_manager.create_room(request.payload) {
                    log::trace!("ルーム作成に失敗。：{}", e);
                }
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
                // update_user(&mut rooms, request.payload)
                //     .expect("ユーザー更新に失敗");
            },
            _ => (),
        };
        
    }

    udp_server.close();
}
