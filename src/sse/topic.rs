use std::time::Duration;

use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    Extension,
};
use futures::stream::Stream;
use serde::Serialize;
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    StreamExt as _,
};

use super::defs::SharedTopicTX;

pub async fn handler(
    Extension(tx): Extension<SharedTopicTX>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let rx = tx.subscribe();

    let stream = BroadcastStream::new(rx);
    let stream = stream.map(|event| {
        tracing::info!("event: {:?}", event);

        event.map(|topic_data| Event::default().data(serde_json::to_string(&topic_data).unwrap()))
    });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(1)))
}
