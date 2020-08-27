use crate::domain::user::{User, Credentials, AuthResult, UserId};
use crate::port::user_port::UserPort;

pub struct UserGateway;

impl UserPort for UserGateway {
    fn login(self, credentials: Credentials) -> AuthResult<User> {
        let mut user =  User {
            userid: UserId::new(),
            username: String::new(),
            name: String::new(),
            surname: String::new(),
            email: String::new()
        };
        match credentials.nameOrEmail {
            Username(username) => user.username = username.to_owned(),
            Email(email) => user.email = email.to_owned(),
        }
        user
    }
}
