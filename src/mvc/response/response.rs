use std::collections::HashMap;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Response {
    pub header: HashMap<String, String>,
    pub payload: String,
}

impl Response {
    pub fn empty() -> Response {
        Response {
            header: HashMap::new(),
            payload: String::from(""),
        }
    }
}
