use axum::{
    extract::Path,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    Extension,
};
use futures::stream::Stream;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    StreamExt as _,
};

use super::defs::SharedReplyChannels;

pub async fn handler(
    Path(id): Path<String>,
    Extension(channels): Extension<SharedReplyChannels>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let mut channels = channels.lock().await;

    let tx = channels.entry(id.clone()).or_insert_with(|| {
        tracing::debug!("Creating new channel for {}", id);

        broadcast::channel(1).0
    });

    let rx = tx.subscribe();

    let stream = BroadcastStream::new(rx);
    let stream = stream.map(|event| {
        event.map(|reply_data| Event::default().data(serde_json::to_string(&reply_data).unwrap()))
    });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(1)))
}
