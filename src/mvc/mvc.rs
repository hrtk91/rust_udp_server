#[path = "middleware/middleware.rs"]
pub mod middleware;
#[path = "request/request.rs"]
pub mod request;
#[path = "response/response.rs"]
pub mod response;
#[path = "controller/controller.rs"]
pub mod controller;
pub mod util;

use std::any::Any;
use std::collections::HashMap;
use middleware::Middleware;
use request::Request;
use util::AsAny;

pub struct Mvc {
    singletons: HashMap<String, Box<dyn Any>>,
    transients: HashMap<String, Box<dyn Any>>,
}

impl Mvc {
    pub fn new() -> Mvc {
        Mvc {
            singletons: HashMap::new(),
            transients: HashMap::new(),
        }
    }

    pub fn add_singleton(&mut self, name: &str, instance: Box<dyn Any>) -> &mut Mvc {
        self.singletons.insert(name.to_string(), instance);
        self
    }

    pub fn add_transient<F>(&mut self, name: &str, func: F) -> &mut Mvc
        where F: Fn() -> Box<dyn Any> + 'static {
        let boxed = Box::new(func) as Box<dyn Fn() -> Box<dyn Any>>;
        let as_any = Box::new(boxed) as Box<dyn Any>;
        self.transients.insert(name.to_string(), as_any);
        self
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn Any>> {
        self.transients.get(key)
    }

    pub fn invoke_middlewares(&mut self, request: &mut Request) -> Result<(), String> {
        let singletons: Vec<&Box<dyn Middleware>> = self.singletons.iter()
            .filter(|(_key, value)| value.is::<Box<dyn Middleware>>())
            .map(|(_key, value)| value.downcast_ref::<Box<dyn Middleware>>().unwrap())
            .collect();
        let transients: Vec<&Box<dyn Fn() -> Box<dyn Middleware>>> = self.transients.iter()
            .filter(|(_key, value)| value.is::<Box<dyn Fn() -> Box<dyn Middleware>>>())
            .map(|(_key, value)| value.downcast_ref::<Box<dyn Fn() -> Box<dyn Middleware>>>().unwrap())
            .collect();
        
        for middleware in singletons {
            middleware.execute(request)?;
        }

        for func in transients {
            let middleware = func();
            middleware.execute(request)?;
        }

        Ok(())
    }
}
