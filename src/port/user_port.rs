use crate::domain::user;
use async_trait::async_trait;
use seed::prelude::wasm_bindgen;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
}

pub type AuthResult<T> = Result<T, AuthError>;

#[async_trait]
pub trait UserPort {
    async fn login(&self, credentials: &user::Credentials) -> AuthResult<user::User>;
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
