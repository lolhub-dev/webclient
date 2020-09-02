use crate::domain::user::{Credentials, UNameOrEmail, User, UserId};
use crate::{
    port::user_port::{AuthError, AuthResult},
    utils,
};
use seed::fetch::fetch;
use seed::log;

pub struct MockUserGateway;

const MOCK_DATA_PATH: &str = "mock_user.json";

async fn get_users() -> AuthResult<Vec<User>> {
    // @TODO: Convert unwrap's into expect to provide more sensible error messages
    let file = fetch(utils::mock_path(MOCK_DATA_PATH))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(&file[..]).map_err(|_| AuthError::NetworkError)
}

fn find_user(
    users: &Vec<User>,
    name_or_email: &UNameOrEmail,
) -> Option<User> {
    let index = users.iter().position(|user| match name_or_email {
        UNameOrEmail::Username(uname) => &user.username == uname,
        UNameOrEmail::Email(email) => &user.email == email,
    });
    index.map_or(None, |index| Some(users[index].clone()))
}

impl MockUserGateway {
    pub async fn login(credentials: &Credentials) -> AuthResult<User> {
        let users = get_users().await?;

        log!(users);

        let user = find_user(&users, &credentials.name_or_email);
        user.ok_or(AuthError::InvalidCredentials)
    }

    pub async fn logout() -> AuthResult<()> {
        Ok(())
    }

    pub async fn register(
        username: &str,
        name: &str,
        surname: &str,
        email: &str,
        _password: &str,
    ) -> AuthResult<User> {
        let uname_taken =
            MockUserGateway::username_taken(username).await?;
        let email_taken = MockUserGateway::email_taken(email).await?;
        if uname_taken {
            Err(AuthError::UsernameTaken)
        } else if email_taken {
            Err(AuthError::EmailTaken)
        } else {
            Ok(User::new(
                UserId::new_v4(),
                String::from(username),
                String::from(name),
                String::from(surname),
                String::from(email),
            ))
        }
    }

    async fn identifier_taken(
        name_or_email: &UNameOrEmail,
    ) -> AuthResult<bool> {
        let users = get_users().await?;
        Ok(find_user(&users, &name_or_email).is_some())
    }

    pub async fn username_taken(username: &str) -> AuthResult<bool> {
        MockUserGateway::identifier_taken(&UNameOrEmail::Username(
            String::from(username),
        ))
        .await
    }

    pub async fn email_taken(email: &str) -> AuthResult<bool> {
        MockUserGateway::identifier_taken(&UNameOrEmail::Email(
            String::from(email),
        ))
        .await
    }
}
