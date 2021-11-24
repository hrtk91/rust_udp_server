use serde_json;
use crate::model::room::{ Room };
use crate::mvc::request::payload::{ UserData };

pub struct RoomManager {
    rooms: Vec<Room>,
}

impl RoomManager {
    pub fn new() -> RoomManager {
        RoomManager {
            rooms: Vec::new(),
        }
    }

    pub fn create_room(&mut self, payload: &str) -> Result<String, String> {
        let mut room: Room = match serde_json::from_str(&payload) {
            Ok(json) => json,
            Err(_) =>  return Err("json serialize failed".to_string()),
        };

        if let None = room.users {
            room.users = Some(Vec::new());
        }

        if self.rooms.iter().any(|x| x.name == room.name) {
            Err("duplicate room name".to_string())
        } else {
            self.rooms.push(room);
            log::trace!("rooms: {:?}", self.rooms);

            match serde_json::to_string(&self.rooms) {
                Ok(json) => Ok(json),
                Err(_) => return Err("failed serialize".to_string()),
            }
        }
    }

    pub fn add_user(&mut self, payload: &str) -> Result<(), String> {
        let user_data: UserData = match serde_json::from_str(&payload) {
            Ok(json) => json,
            Err(_) =>  return Err("json serialize failed".to_string()),
        };
    
        let room = match self.rooms.iter_mut().find(|x| x.name == user_data.room_name) {
            Some(room) => room,
            None => return Err("room not found : {:?}".to_string()),
        };

        room.add_user(user_data)
    }

    pub fn update_user(&mut self, payload: &str) -> Result<(), String> {
        let user_data: UserData = match serde_json::from_str(&payload) {
            Ok(json) => json,
            Err(_) =>  return Err("json serialize failed".to_string()),
        };
    
        let room = match self.rooms.iter_mut().find(|x| x.name == user_data.room_name) {
            Some(room) => room,
            None => return Err("room not found : {:?}".to_string()),
        };

        room.update_user(&user_data)
    }

    pub fn remove_user(&mut self, payload: &str) -> Result<(), String> {
        let user_data: UserData = match serde_json::from_str(&payload) {
            Ok(json) => json,
            Err(_) =>  return Err("json serialize failed".to_string()),
        };
    
        let room = match self.rooms.iter_mut().find(|x| x.name == user_data.room_name) {
            Some(room) => room,
            None => return Err("room not found : {:?}".to_string()),
        };

        room.remove_user(&user_data)
    }
}
