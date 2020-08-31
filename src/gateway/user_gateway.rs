use crate::domain::user::{User, UserId};
use crate::port::user_port::AuthResult;

pub struct UserGateway;


impl UserGateway {

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
