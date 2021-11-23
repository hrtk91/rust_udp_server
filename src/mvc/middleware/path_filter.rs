use super::{ Middleware, Request };
use super::super::AsAny;
use http::Uri;
use std::any::Any;

#[derive(Debug)]
pub struct PathFilter;

impl PathFilter {
    pub fn new() -> PathFilter {
        PathFilter {}
    }
}

impl Middleware for PathFilter {
    fn execute(&self, request: &mut Request) -> Result<(), String> {
        log::info!("start");
        let req_path = match request.header.get("req_path") {
            Some(req_path) => req_path,
            None => return Err("req_path_is_empty".to_string()),
        };

        let uri = match req_path.parse::<Uri>() {
            Ok(uri) => uri,
            Err(_) => return Err("req_path_not_uri".to_string()),
        };
        request.header.insert("path".to_string(), uri.path().to_string());

        let split = uri.path().split("/").map(|x| x.to_string()).collect::<Vec<String>>();

        let controller_name = split[1].clone();
        request.header.insert("controller_name".to_string(), controller_name.clone());
        request.header.insert("controller_name_without_prefix".to_string(),
            regex::Regex::new("Controller$").unwrap().replace(&controller_name, "").to_string());

        log::info!("end");
        Ok(())
    }

    fn as_middleware(self) -> Box<dyn Middleware> {
        Box::new(self) as Box<dyn Middleware>
    }
}

impl AsAny for PathFilter {
    fn as_any(self) -> Box<dyn Any> {
        Box::new(self.as_middleware()) as Box<dyn Any>
    }
}
