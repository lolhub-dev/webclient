use ulid::Ulid;

pub type UserId = Ulid;

#[derive(Debug)]
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
    nameOrEmail: UNameOrEmail,
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
