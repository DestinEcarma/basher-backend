use std::{collections::HashMap, sync::Arc};

use async_graphql::ID;
use futures::lock::Mutex;
use serde::Serialize;
use tokio::sync::broadcast::Sender;

pub type TopicTX = Sender<TopicData>;
pub type SharedTopicTX = Arc<TopicTX>;

pub type ReplyTX = Sender<ReplyData>;
pub type SharedReplyChannels = Arc<Mutex<HashMap<String, ReplyTX>>>;

#[derive(Serialize, Clone, Debug)]
pub struct TopicData {
    id: ID,
}

impl TopicData {
    pub fn new(id: ID) -> Self {
        Self { id }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ReplyData {
    id: ID,
    kind: String,
    class: String,
}

impl ReplyData {
    pub fn new(id: ID, kind: &str, class: &str) -> Self {
        Self {
            id,
            kind: kind.to_string(),
            class: class.to_string(),
        }
    }
}
