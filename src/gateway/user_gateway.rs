use crate::domain::user::{Credentials, User, UserId};
use crate::port::user_port::{AuthResult, UserPort};

pub struct UserGateway;

impl UserPort for UserGateway {
    fn login(&self, credentials: Credentials) -> AuthResult<User> {
        Ok(User::default())
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
        Ok(false)
    }
}
