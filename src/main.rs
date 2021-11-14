use std::result::{ Result };
use std::sync::mpsc::{ TryRecvError };
use serde::{ Serialize, Deserialize };
mod udp_server;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Request {
    req_type: Option<String>,
    payload: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct UserData {
    room_name: Option<String>,
    user_id: Option<String>,
    x: Option<u32>,
    y: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Room {
    name: Option<String>,
    password: Option<String>,
    users: Option<Vec<UserData>>,
}

fn find_user<'a>(rooms: &'a mut Vec<Room>, user_data: &UserData) -> Result<&'a mut UserData, &'a str> {
    let room = match rooms.iter_mut().find(|x| x.name == user_data.room_name) {
        Some(room) => room,
        None => return Err("room not found"),
    };

    match &mut room.users {
        Some(users) => match users.iter_mut().find(|x| x.user_id == user_data.user_id) {
            Some(user) => Ok(user),
            None => return Err("user not found"),
        },
        None => return Err("room has not users"),
    }
}

fn add_user(rooms: &mut Vec<Room>, payload: Option<String>) -> Result<(), &str> {
    let payload = match payload {
        Some(payload) => payload,
        None => return Err("payload is None"),
    };

    let user_data: UserData = match serde_json::from_str(&payload) {
        Ok(json) => json,
        Err(_) =>  return Err("json serialize failed"),
    };

    let room = match rooms.iter_mut().find(|x| x.name == user_data.room_name) {
        Some(room) => room,
        None => return Err("room not found : {:?}"),
    };

    let users = match &mut room.users {
        Some(users) => users,
        None => return Err("room.users is not exists"),
    };

    log::trace!("room \"{}\" add user : {:?}", room.name.as_ref().unwrap(), user_data);

    users.push(user_data);

    Ok(())
}

fn update_user(rooms: &mut Vec<Room>, payload: Option<String>) -> Result<(), &str> {
    let payload = match payload {
        Some(payload) => payload,
        None => return Err("payload is None"),
    };

    let user_data: UserData = match serde_json::from_str(&payload) {
        Ok(json) => json,
        Err(_) =>  return Err("json serialize failed"),
    };

    let mut user = match find_user(rooms, &user_data) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    log::trace!("before user:{:?}", user);

    user.x = user_data.x;
    user.y = user_data.y;

    Ok(())
}

fn create_room(rooms: &mut Vec<Room>, payload: Option<String>) -> std::result::Result<(), String> {
    let payload = match payload {
        Some(payload) => payload,
        None => return Err("payload is None".to_string()),
    };

    let mut room: Room = match serde_json::from_str(&payload) {
        Ok(json) => json,
        Err(_) =>  return Err("json serialize failed".to_string()),
    };

    if let None = room.users {
        room.users = Some(Vec::new());
    }

    if rooms.iter().any(|x| x.name == room.name) {
        Err("failed create room".to_string())
    } else {
        rooms.push(room);
        log::trace!("rooms: {:?}", rooms);
        Ok(())
    }
}

fn main() {
    env_logger::init();

    let udp_server = udp_server::listen("0.0.0.0:8080").unwrap();
    let mut rooms: Vec<Room> = std::vec::Vec::new();

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
                if let Err(e) = create_room(&mut rooms, request.payload) {
                    log::trace!("ルーム作成に失敗。：{}", e);
                }
            },
            "add_user" => {
                if let Err(e) = add_user(&mut rooms, request.payload) {
                    log::trace!("ユーザー追加に失敗。：{}", e);
                }
            },
            "update_user" => {
                if let Err(e) = update_user(&mut rooms, request.payload) {
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
