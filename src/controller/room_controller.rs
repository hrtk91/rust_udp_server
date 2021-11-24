use std::any::Any;
use std::collections::HashMap;
use crate::mvc::{ Mvc, Scope };
use crate::mvc::util::AsAny;
use crate::mvc::response::Response;
use crate::mvc::request::Request;
use crate::mvc::controller::{ Controller };
use crate::model::room_manager::RoomManager;

pub struct RoomController;


impl RoomController {
    pub fn new() -> RoomController {
        RoomController {}
    }

    pub fn create(&mut self, request: &Request, mvc: &mut Mvc) -> Result<Response, String> {
        let room_manager = match mvc.get(crate::nameof!(RoomManager), Scope::Singleton) {
            Some(boxed) => boxed.downcast_mut::<RoomManager>().unwrap(),
            None => return Err("failed get RoomManager".to_string()),
        };

        let json = match room_manager.create_room(&request.payload) {
            Ok(json) => json,
            Err(e) => return Err(e),
        };

        log::info!("ルーム作成に成功");

        let mut header = HashMap::new();
        header.insert("status_code".to_string(), "ok".to_string());

        Ok(Response {
            header: header,
            payload: json,
        })
    }

    pub fn add_user(&mut self, request: &Request, mvc: &mut Mvc) -> Result<Response, String> {
        let room_manager = match mvc.get(crate::nameof!(RoomManager), Scope::Singleton) {
            Some(boxed) => boxed.downcast_mut::<RoomManager>().unwrap(),
            None => return Err("failed get RoomManager".to_string()),
        };

        if let Err(e) = room_manager.add_user(&request.payload) {
            return Err(e)
        };

        log::info!("ルームにユーザーを追加成功");

        let mut header = HashMap::new();
        header.insert("status_code".to_string(), "ok".to_string());

        Ok(Response {
            header: header,
            payload: "".to_string(),
        })
    }

    pub fn update_user(&mut self, request: &Request, mvc: &mut Mvc) -> Result<Response, String> {
        let room_manager = match mvc.get(crate::nameof!(RoomManager), Scope::Singleton) {
            Some(boxed) => boxed.downcast_mut::<RoomManager>().unwrap(),
            None => return Err("failed get RoomManager".to_string()),
        };

        if let Err(e) = room_manager.update_user(&request.payload) {
            return Err(e)
        };

        log::info!("ルームのユーザー更新成功");

        let mut header = HashMap::new();
        header.insert("status_code".to_string(), "ok".to_string());

        Ok(Response {
            header: header,
            payload: "".to_string(),
        })
    }

    pub fn remove_user(&mut self, request: &Request, mvc: &mut Mvc) -> Result<Response, String> {
        let room_manager = match mvc.get(crate::nameof!(RoomManager), Scope::Singleton) {
            Some(boxed) => boxed.downcast_mut::<RoomManager>().unwrap(),
            None => return Err("failed get RoomManager".to_string()),
        };

        if let Err(e) = room_manager.remove_user(&request.payload) {
            return Err(e)
        };

        log::info!("ルームのユーザー削除成功");

        let mut header = HashMap::new();
        header.insert("status_code".to_string(), "ok".to_string());

        Ok(Response {
            header: header,
            payload: "".to_string(),
        })
    }
}

impl Controller for RoomController {
    fn invoke_middlewares(&mut self, _request: &mut Request) -> Result<(), String> {
        Ok(())
    }
    
    fn action(&mut self, request: &mut Request, mvc: &mut Mvc) -> Result<Response, String> {

        let path = match request.header.get("path") {
            Some(path) => path.to_lowercase(),
            None => return Err("path is None".to_string()),
        };

        let response = match path.as_str() {
            "/room/create" => self.create(request, mvc)?,
            "/room/user/add" => self.add_user(request, mvc)?,
            "/room/user/update" => self.update_user(request, mvc)?,
            "/room/user/remove" => self.remove_user(request, mvc)?,
            _ => {
                let mut header = HashMap::new();
                header.insert("status_code".to_string(), "NotFound".to_string());
                Response{
                    header: header,
                    payload: "".to_string(),
                }
            }
        };

        Ok(response)
    }

    fn as_controller(self) -> Box<dyn Controller> {
        Box::new(self)
    }
}

impl AsAny for RoomController {
    fn as_any(self) -> Box<dyn Any> {
        Box::new(self.as_controller())
    }
}
