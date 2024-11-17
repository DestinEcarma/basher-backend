mod defs;
mod mutation;
mod query;

pub use mutation::RootMutation;
pub use query::RootQuery;

use async_graphql::EmptySubscription;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{response, Extension, Router};
use tower_cookies::Cookies;

pub type ApiSchema = async_graphql::Schema<RootQuery, RootMutation, EmptySubscription>;

pub async fn graphiql() -> impl response::IntoResponse {
    response::Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}

pub async fn handler(
    cookies: Cookies,
    schema: Extension<ApiSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    req = req.data(cookies);

    schema.execute(req).await.into()
}
