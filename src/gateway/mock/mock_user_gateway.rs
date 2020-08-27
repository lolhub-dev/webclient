use crate::domain::user::{Credentials, UNameOrEmail, User, UserId};
use crate::port::user_port::{AuthError, AuthResult, UserPort};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::Result;

pub struct MockUserGateway;

fn get_users() -> Vec<User> {
    let filepath = Path::new("../../../test/mock_user.json");
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let users: Vec<User> = serde_json::from_reader(reader)?;
    users
}

impl UserPort for MockUserGateway {
    fn login(self, credentials: Credentials) -> AuthResult<User> {
        let users = get_users();
        let ret_user = users
            .iter()
            .filter(|user: User| match credentials.name_or_email {
                UNameOrEmail::Username(uname) => user.username == uname,
                UNameOrEmail::Email(email) => user.email == email,
            })
            .next();

        match ret_user {
            Some(user) => Ok(user),
            None => Err(AuthError::InvalidCredentials),
        }
    }

    fn logout(self) -> AuthResult<()> {
        Ok(())
    }

    fn register(
        self,
        username: String,
        name: String,
        surname: String,
        email: String,
        password: String,
    ) -> AuthResult<user::User> {
        Ok(User {
            userid: UserId::new(),
            username,
            name,
            surname,
            email,
        })
    }

    fn username_taken(self, username: String) -> AuthResult<bool> {
        let users = get_users();
        users.iter().any(|user: User| user.username == username)
    }
}
