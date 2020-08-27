use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub type UserId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    userid: UserId,
    username: String,
    name: String,
    surname: String,
    email: String,
}

#[derive(Debug)]
pub enum UNameOrEmail {
    Username(String),
    Email(String)
}

#[derive(Debug)]
pub struct Credentials {
    name_or_email: UNameOrEmail,
    password: String
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
