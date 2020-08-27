use crate::domain::user;

pub enum AuthError {
    InvalidCredentials,
}

pub type AuthResult<T> = Result<T, AuthError>;

pub trait UserPort {
    fn login(&self, credentials: user::Credentials) -> AuthResult<user::User>;
    fn logout(&self) -> AuthResult<()>;
    fn register(
        &self,
        username: String,
        name: String,
        surname: String,
        email: String,
        password: String,
    ) -> AuthResult<user::User>;
    fn username_taken(&self, username: String) -> AuthResult<bool>;
}
