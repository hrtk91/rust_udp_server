use std::any::Any;
use crate::mvc::util::AsAny;
use crate::mvc::response::Response;
use crate::mvc::request::Request;
use crate::mvc::controller::{ Controller };

pub struct RoomController;

impl RoomController {
    pub fn new() -> RoomController {
        RoomController {}
    }

    pub fn create(&mut self) -> Result<Response, String> {
        log::info!("create");
        Ok(Response::empty())
    }

    pub fn update(&mut self) -> Result<Response, String> {
        log::info!("update");
        Ok(Response::empty())
    }

    pub fn remove(&mut self) -> Result<Response, String> {
        log::info!("remove");
        Ok(Response::empty())
    }
}

impl Controller for RoomController {
    fn invoke_middlewares(&mut self, _request: &mut Request) -> Result<(), String> {
        Ok(())
    }
    
    fn action(&mut self, request: &mut Request) -> Result<Response, String> {

        let path = match request.header.get("path") {
            Some(path) => path.to_lowercase(),
            None => return Err("path is None".to_string()),
        };

        let response = match path.as_str() {
            "/room/create" => self.create()?,
            "/room/update" => self.update()?,
            "/room/remove" => self.remove()?,
            _ => return Err("path is invalid".to_string())
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
