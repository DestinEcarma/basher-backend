pub mod defs;

mod reply;
mod topic;

use axum::{extract::Path, routing::get, Extension, Router};
use defs::{SharedReplyChannels, SharedTopicTX, TopicTX};
use std::sync::{Arc, Mutex};
use tower::ServiceBuilder;

pub fn router(topic_tx: &defs::SharedTopicTX, reply_channels: &SharedReplyChannels) -> Router {
    let topic_route_wrapper = |extension: Extension<SharedTopicTX>| topic::handler(extension);
    let reply_route_wrapper = |path: Path<String>, extension: Extension<SharedReplyChannels>| {
        reply::handler(path, extension)
    };

    Router::new()
        .route("/topic", get(topic_route_wrapper))
        .route("/topic/:id", get(reply_route_wrapper))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(topic_tx.clone()))
                .layer(Extension(reply_channels.clone())),
        )
}
