#![allow(unused)]

mod auth;
mod config;
mod db;
mod error;
mod graphql;
mod miscs;
mod sse;

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
use db::table::{Reply, Topic};
use futures::lock::Mutex;
use graphql::{RootMutation, RootQuery};
use sse::defs::{ReplyData, ReplyTX, SharedReplyChannels, TopicData};
use std::collections::HashMap;
use std::default;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
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

    let db = Arc::new(db::get_connection().await?);

    let (topic_tx, _rx) = broadcast::channel::<TopicData>(1);
    let topic_tx = Arc::new(topic_tx);

    let reply_channels: SharedReplyChannels =
        Arc::new(Mutex::new(HashMap::<String, ReplyTX>::new()));

    let app = Router::new()
        .nest("/sse", sse::router(&topic_tx, &reply_channels))
        .nest("/graphql", graphql::router(&db, &topic_tx, &reply_channels))
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
