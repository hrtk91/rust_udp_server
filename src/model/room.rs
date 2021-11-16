use serde::{ Serialize, Deserialize };
use crate::model::request::payload::{ UserData };

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Room {
    pub name: Option<String>,
    pub password: Option<String>,
    pub users: Option<Vec<UserData>>,
}

impl Room {
    pub fn find_user<'a>(&'a mut self, user_data: &UserData) -> Result<&'a mut UserData, &'a str> {
        match &mut self.users {
            Some(users) => match users.iter_mut().find(|x| x.user_id == user_data.user_id) {
                Some(user) => Ok(user),
                None => return Err("user not found"),
            },
            None => return Err("users is None"),
        }
    }

    pub fn add_user(&mut self, user_data: UserData) -> Result<(), &str> {
        let users = match &mut self.users {
            Some(users) => users,
            None => return Err("users is None"),
        };

        log::trace!("room \"{}\" add user : {:?}", self.name.as_ref().unwrap(), user_data);

        users.push(user_data);

        Ok(())
    }

    pub fn update_user(&mut self, user_data: &UserData) -> Result<(), &str> {
        if let None = user_data.x {
            return Err("user_data.x is None");
        } else if let None = user_data.y {
            return Err("user_data.y is None");
        };

        let user = self.find_user(user_data)?;
    
        log::trace!("before user:{:?}", user);
    
        user.x = user_data.x;
        user.y = user_data.y;

        log::trace!("after user:{:?}", user);
    
        Ok(())
    }

    pub fn remove_user(&mut self, user_data: &UserData) -> Result<(), &str> {
        if let None = user_data.x {
            return Err("user_data.x is None");
        } else if let None = user_data.y {
            return Err("user_data.y is None");
        };

        match &mut self.users {
            Some(users) => {
                if let Some(idx) = users.iter_mut().position(|x|
                    x.user_id == user_data.user_id || x.room_name == user_data.room_name) {
                    users.remove(idx);
                } else {
                    return Err("user not found");
                }
                Ok(())
            },
            None => return Err("user not found :{:?}"),
        }
    }
}
