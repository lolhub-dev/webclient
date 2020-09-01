use crate::domain::user::{Credentials, User};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    NetworkError,
}

pub type AuthResult<T> = Result<T, AuthError>;

pub trait UserPort {
    fn login(
        &self,
        credentials: &Credentials,
    ) -> AuthResult<User>;
    fn logout(&self) -> AuthResult<()>;
    fn register(
        &self,
        username: String,
        name: String,
        surname: String,
        email: String,
        password: String,
    ) -> AuthResult<User>;
    fn username_taken(&self, username: String) -> AuthResult<bool>;
}
