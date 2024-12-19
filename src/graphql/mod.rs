mod defs;
mod mutation;
mod query;

pub use mutation::RootMutation;
pub use query::RootQuery;

use crate::db::defs::SharedDB;
use crate::sse::defs::{SharedReplyChannels, SharedTopicTX};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Request;
use axum::routing::post;
use axum::{body::Body, Extension, Router};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use defs::ApiSchema;
use tower::ServiceBuilder;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::trace::TraceLayer;

pub async fn handler(
    cookies: Cookies,
    schema: Extension<ApiSchema>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    req = req.data(cookies);
    req = req.data(auth_header);

    schema.execute(req).await.into()
}

pub fn router(
    db: &SharedDB,
    topic_tx: &SharedTopicTX,
    reply_channels: &SharedReplyChannels,
) -> Router {
    let schema = ApiSchema::build(Default::default(), Default::default(), Default::default())
        .data(db.clone())
        .data(topic_tx.clone())
        .data(reply_channels.clone())
        .finish();

    Router::new().route("/", post(handler)).layer(
        ServiceBuilder::new()
            .layer(CookieManagerLayer::new())
            .layer(Extension(schema))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|_: &Request<Body>| tracing::info_span!("Graphql")),
            ),
    )
}
