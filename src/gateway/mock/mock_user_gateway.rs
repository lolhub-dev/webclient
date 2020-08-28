use crate::domain::user::{Credentials, UNameOrEmail, User, UserId};
use crate::{mock_path, port::user_port::{AuthError, AuthResult, UserPort}};
use futures::executor;
use seed::{fetch::fetch, log, prelude::FetchError};
use serde_json::error::Error as SerdeError;

pub struct MockUserGateway;

const MOCK_DATA_PATH: &str = "mock_user.json";

async fn get_users() -> Result<Vec<User>, FetchError> {
    let file = &fetch(mock_path(MOCK_DATA_PATH)).await?.text().await?;
    let users: Result<Vec<User>, SerdeError> = serde_json::from_str(file);
    users.map_err(|err| FetchError::SerdeError(err))
}

impl UserPort for MockUserGateway {
    //TODO: as soon as async is supported in traits use it !!!
    fn login(&self, credentials: &Credentials) -> AuthResult<User> {
        //TODO: safe handling of future and the Result type and its possible errors
        let users = executor::block_on(get_users()).unwrap();
        let ret_user = users
            .into_iter()
            .filter(|user| match &credentials.name_or_email {
                UNameOrEmail::Username(uname) => user.username == *uname,
                UNameOrEmail::Email(email) => user.email == *email,
            })
            .next();

        match ret_user {
            Some(user) => Ok(user),
            None => Err(AuthError::InvalidCredentials),
        }
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
        //TODO: safe handling of future and the Result type and its possible errors
        let users = executor::block_on(get_users()).unwrap();
        Ok(users.into_iter().any(|user| user.username == username))
    }
}
