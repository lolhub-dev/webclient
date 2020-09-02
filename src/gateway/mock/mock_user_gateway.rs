use crate::domain::user::{Credentials, UNameOrEmail, User};
use crate::{
    port::user_port::{AuthError, AuthResult},
    utils,
};
use seed::{fetch::fetch, log};
use serde_json::error::Error as JsonError;

pub struct MockUserGateway;

const MOCK_DATA_PATH: &str = "mock_user.json";

async fn get_users() -> Result<Vec<User>, JsonError> {
    let file = fetch(utils::mock_path(MOCK_DATA_PATH))
            .await
            //.map_err(|_| AuthError::NetworkError)
            .unwrap()
            .text()
            .await
            //.map_err(|_| AuthError::NetworkError)
            .unwrap();

    let users: Result<Vec<User>, JsonError> =
        serde_json::from_str(&file[..]);
    users
}

impl MockUserGateway {
    pub async fn login(credentials: Credentials) -> AuthResult<User> {
        let users = get_users().await;

        log!(
            "matching against {} and {}",
            credentials.name_or_email,
            credentials.password
        );
        // TODO: how to model username check here ? Should this be done here, or is this a server
        // thing
        let ret_user = users.map(|mut users| {
            users.retain(|user| match &credentials.name_or_email {
                UNameOrEmail::Username(uname) => user.username == *uname,
                UNameOrEmail::Email(email) => user.email == *email,
            });
            users.pop()
        });

        let res = match ret_user {
            Ok(Some(user)) => Ok(user),
            _ => Err(AuthError::InvalidCredentials),
        };
        res
    }
}
