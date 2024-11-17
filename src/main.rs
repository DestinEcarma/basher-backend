#![allow(unused)]

mod auth;
mod config;
mod db;
mod error;
mod graphql;
mod miscs;

pub use crate::config::config;
pub use crate::error::{ClientError, Error, Result};

use async_graphql::EmptySubscription;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Extension, Router,
};
use graphql::{ApiSchema, RootMutation, RootQuery};
use std::default;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_target(false)
            .without_time()
            .finish(),
    )
    .unwrap();

    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("{panic_info}");
    }));

    let db = db::get_connection().await?;

    let schema = ApiSchema::build(Default::default(), Default::default(), Default::default())
        .data(db)
        .finish();

    let graphql_router = Router::new()
        .route("/", get(graphql::graphiql).post(graphql::handler))
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(Extension(schema))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|_: &Request<Body>| tracing::info_span!("Graphql")),
                ),
        );

    let app = Router::new()
        .nest("/graphql", graphql_router)
        .layer(cors_middleware()?)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| {
                    tracing::info_span!("Http", request_id = %surrealdb::sql::Id::rand())
                })
                .on_request(|req: &Request<Body>, _span: &Span| {
                    tracing::info!(method = %req.method(), path = %req.uri().path());
                    // tracing::debug!("Headers: {:#?}", req.headers());
                })
                .on_response(|res: &Response<Body>, time: Duration, _span: &Span| {
                    tracing::info!(status = %format!("({})", res.status()), time = %format!("{:.2?}", time));
                    // tracing::debug!("Headers: {:#?}", req.headers());
                })
                .on_failure(
                    |error: ServerErrorsFailureClass, _time: Duration, _span: &Span| {
                        tracing::error!("{error}");
                    },
                ),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    let span = tracing::info_span!("Listening");
    let _enter = span.enter();

    tracing::info!(address = %listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

fn cors_middleware() -> Result<CorsLayer> {
    Ok(CorsLayer::new()
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::SET_COOKIE,
            header::COOKIE,
            header::ACCEPT,
        ])
        .allow_credentials(true)
        .allow_methods([Method::POST, Method::GET, Method::PATCH, Method::DELETE])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()))
}
