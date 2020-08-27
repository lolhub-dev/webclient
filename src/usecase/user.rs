use crate::domain::user;
use crate::port::user_port::{AuthResult, UserPort};

pub fn login_user(gateway: &impl UserPort, credentials: &user::Credentials) -> AuthResult<user::User> {
    gateway.login(credentials)
}
