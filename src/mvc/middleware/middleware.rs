mod path_filter;
use super::request::Request;
pub use path_filter::PathFilter;

pub trait Middleware {
    fn execute(&self, controller: &mut Request) -> Result<(), String>;
    fn as_middleware(self) -> Box<dyn Middleware>;
}
