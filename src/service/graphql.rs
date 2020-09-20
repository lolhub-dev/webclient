pub mod graph_ql {
    use graphql_client::{GraphQLQuery, Response as GQLResponse};
    use seed::{
        fetch,
        prelude::{Method, Request},
    };
    use serde::{Deserialize, Serialize};

    const API_URL: &str = "http://localhost:3000/";

    type Code = String;

    // ------ ------
    //    GraphQL
    // ------ ------

    // defines a macro, that builds a pub struct for a graphql file
    macro_rules! generate_query {
        ($query:ident) => {
            #[derive(GraphQLQuery)]
            #[graphql(
                schema_path = "graphql/schema.graphql",
                query_path = "graphql/queries.graphql",
                response_derives = "Debug"
            )]
            pub struct $query;
        };
    }
    // generate the query struct and name it MLogin
    generate_query!(MLogin);

    pub async fn send_graphql_request<V, T>(
        variables: &V,
    ) -> fetch::Result<T>
    where
        V: Serialize,
        T: for<'de> Deserialize<'de> + 'static,
    {
        Request::new(API_URL)
            .method(Method::Post)
            .json(variables)?
            .fetch()
            .await?
            .check_status()?
            .json()
            .await
    }
}
