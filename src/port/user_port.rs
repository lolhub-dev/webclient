use crate::domain::user::{User, Credentials};
use async_trait::async_trait;
use seed::prelude::wasm_bindgen;
use serde_json::error::Error as SerdeError;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    NetworkError
}

pub type AuthResult<T>  = Result<T, AuthError>;

#[async_trait]
pub trait UserPort {
    async fn login(&self, credentials: &Credentials) -> AuthResult<User>;
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
