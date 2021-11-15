use serde_json;
use crate::model::room::{ Room };
use crate::model::request::payload::{ UserData };

pub struct RoomManager {
    rooms: Vec<Room>,
}

impl RoomManager {
    pub fn new() -> RoomManager {
        RoomManager {
            rooms: Vec::new(),
        }
    }

    pub fn create_room(&mut self, payload: Option<String>) -> Result<(), &str> {
        let payload = match payload {
            Some(payload) => payload,
            None => return Err("payload is None"),
        };

        let mut room: Room = match serde_json::from_str(&payload) {
            Ok(json) => json,
            Err(_) =>  return Err("json serialize failed"),
        };

        if let None = room.users {
            room.users = Some(Vec::new());
        }

        if self.rooms.iter().any(|x| x.name == room.name) {
            Err("duplicate room name")
        } else {
            self.rooms.push(room);
            log::trace!("rooms: {:?}", self.rooms);
            Ok(())
        }
    }

    pub fn add_user(&mut self, payload: Option<String>) -> Result<(), &str> {
        let payload = match payload {
            Some(payload) => payload,
            None => return Err("payload is None"),
        };
    
        let user_data: UserData = match serde_json::from_str(&payload) {
            Ok(json) => json,
            Err(_) =>  return Err("json serialize failed"),
        };
    
        let room = match self.rooms.iter_mut().find(|x| x.name == user_data.room_name) {
            Some(room) => room,
            None => return Err("room not found : {:?}"),
        };

        room.add_user(user_data)
    }

    pub fn update_user(&mut self, payload: Option<String>) -> Result<(), &str> {
        let payload = match payload {
            Some(payload) => payload,
            None => return Err("payload is None"),
        };
    
        let user_data: UserData = match serde_json::from_str(&payload) {
            Ok(json) => json,
            Err(_) =>  return Err("json serialize failed"),
        };
    
        let room = match self.rooms.iter_mut().find(|x| x.name == user_data.room_name) {
            Some(room) => room,
            None => return Err("room not found : {:?}"),
        };

        room.update_user(&user_data)
    }
}
