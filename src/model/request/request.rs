pub mod payload;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Request {
    pub req_type: Option<String>,
    pub payload: Option<String>,
}
