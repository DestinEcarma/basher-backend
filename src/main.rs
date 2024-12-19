mod auth;
mod config;
mod db;
mod error;
mod graphql;
mod miscs;
mod sse;

pub use crate::config::config;
pub use crate::error::{ClientError, Error, Result};

use axum::body::Body;
use axum::http::{Request, Response};
use axum::Router;
use futures::lock::Mutex;
use sse::defs::{ReplyTX, SharedReplyChannels, TopicData};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::Span;

async fn app() -> Result<Router> {
    let db = Arc::new(db::get_connection().await?);

    let (topic_tx, _rx) = broadcast::channel::<TopicData>(1);
    let topic_tx = Arc::new(topic_tx);

    let reply_channels: SharedReplyChannels =
        Arc::new(Mutex::new(HashMap::<String, ReplyTX>::new()));

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Ok(Router::new()
        .nest("/sse", sse::router(&topic_tx, &reply_channels))
        .nest("/graphql", graphql::router(&db, &topic_tx, &reply_channels))
        .fallback_service(Router::new().nest_service("/", serve_dir))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_: &Request<Body>| {
                    tracing::debug_span!("Http", request_id = %surrealdb::sql::Id::rand())
                })
                .on_request(|req: &Request<Body>, _span: &Span| {
                    tracing::debug!(method = %req.method(), path = %req.uri().path());
                    // tracing::debug!("Headers: {:#?}", req.headers());
                })
                .on_response(|res: &Response<Body>, time: Duration, _span: &Span| {
                    tracing::debug!(status = %format!("({})", res.status()), time = %format!("{:.2?}", time));
                    // tracing::debug!("Headers: {:#?}", req.headers());
                })
                .on_failure(
                    |error: ServerErrorsFailureClass, _time: Duration, _span: &Span| {
                        tracing::error!("{error}");
                    },
                ),
        )
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("{panic_info}");
    }));

    let app = app().await.unwrap();

    Ok(app.into())
}
