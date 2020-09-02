use crate::domain::user::{Credentials, User, UserId};
use crate::{
    port::user_port::{AuthError, AuthResult},
    service::graphql::graph_ql,
};
use graphql_client::{GraphQLQuery, Response as GQLResponse};

pub struct UserGateway;

impl UserGateway {
    pub async fn login(credentials: Credentials) -> AuthResult<User> {
        let request_body = graph_ql::MLogin::build_query(graph_ql::m_login::Variables {
                username: credentials.name_or_email.into(),
                password: credentials.password,
            });
        graph_ql::send_graphql_request(&request_body)
            .await
            .map_err(|_| AuthError::NetworkError)
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
        Ok(false)
    }
}
