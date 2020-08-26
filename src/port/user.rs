use crate::domain::user;

type AuthResult<T> = Result<T, Error>;

pub trait UserPort {
    fn login(&self, credentials: user::Credentials) -> AuthResult<User>;
    fn logout(&self) -> AuthResult<_>;
    fn register(
        &self,
        username: String,
        name: String,
        surname: String,
        email: String,
        password: String,
    ) -> AuthResult<User>;
}
