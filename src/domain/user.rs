use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub userid: UserId,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub enum UNameOrEmail {
    Username(String),
    Email(String),
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub name_or_email: UNameOrEmail,
    pub password: String,
}

impl User {
    pub fn new(
        userid: UserId,
        username: String,
        name: String,
        surname: String,
        email: String,
    ) -> Self {
        User {
            userid,
            username,
            name,
            surname,
            email,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            userid: UserId::new_v4(),
            username: String::new(),
            name: String::new(),
            surname: String::new(),
            email: String::new(),
        }
    }
}
