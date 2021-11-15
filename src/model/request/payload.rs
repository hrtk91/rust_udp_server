use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserData {
    pub room_name: Option<String>,
    pub user_id: Option<String>,
    pub x: Option<u32>,
    pub y: Option<u32>,
}
