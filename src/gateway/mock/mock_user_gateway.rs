use crate::domain::user::{Credentials, UNameOrEmail, User, UserId};
use crate::port::user_port::{AuthError, AuthResult, UserPort};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct MockUserGateway;

const MOCK_DATA_PATH: &str = "../../../test/mock_user.json";

fn get_users() -> Vec<User> {
    let filepath = Path::new(MOCK_DATA_PATH);
    let file = File::open(filepath).expect("Could not find mock data.");
    let reader = BufReader::new(file);

    let users: Vec<User> = serde_json::from_reader(reader).expect("Could not parse mock data.");
    users
}

impl UserPort for MockUserGateway {
    fn login(&self, credentials: &Credentials) -> AuthResult<User> {
        let users = get_users();
        let ret_user = users
            .into_iter()
            .filter(|user| match credentials.name_or_email {
                UNameOrEmail::Username(uname) => user.username == uname,
                UNameOrEmail::Email(email) => user.email == email,
            })
            .next();

        match ret_user {
            Some(user) => Ok(user),
            None => Err(AuthError::InvalidCredentials),
        }
    }

    fn logout(&self) -> AuthResult<()> {
        Ok(())
    }

    fn register(
        &self,
        username: String,
        name: String,
        surname: String,
        email: String,
        password: String,
    ) -> AuthResult<User> {
        Ok(User::new(UserId::new_v4(), username, name, surname, email))
    }

    fn username_taken(&self, username: String) -> AuthResult<bool> {
        let users = get_users();
        Ok(users.into_iter().any(|user| user.username == username))
    }
}
