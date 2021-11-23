pub mod payload;
use std::collections::HashMap;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Request {
    pub header: HashMap<String, String>,
    pub payload: String,
}

impl Request {
    pub fn empty() -> Request {
        let mut map = HashMap::new();
        map.insert("req_type1".to_string(), "".to_string());
        map.insert("req_type2".to_string(), "".to_string());
        Request {
            header: map,
            payload: String::from(""),
        }
    }
}
