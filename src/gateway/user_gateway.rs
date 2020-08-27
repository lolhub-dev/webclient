use crate::domain::user::{AuthResult, Credentials, User, UserId};
use crate::port::user_port::UserPort;

pub struct UserGateway;

impl UserPort for UserGateway {
    fn login(self, credentials: Credentials) -> AuthResult<User> {
        User {
            userid: UserId::new(),
            username: String::new(),
            name: String::new(),
            surname: String::new(),
            email: String::new(),
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
        Ok(false)
    }
}
