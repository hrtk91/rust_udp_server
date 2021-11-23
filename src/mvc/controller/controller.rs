use super::request::Request;
use super::response::Response;
use std::result::Result;

pub trait Controller {
    fn invoke(&mut self, request: & mut Request) -> Result<Response, String> {
        if let Err(e) = self.invoke_middlewares(request) {
            return Err(e);
        }

        self.action(request)
    }

    fn invoke_middlewares(&mut self, reuqest: &mut Request) -> Result<(), String>;
    fn action(&mut self, request: &mut Request) -> Result<Response, String>;
    fn as_controller(self) -> Box<dyn Controller>;
}
