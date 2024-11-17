mod reply;
mod topic;
mod user;

use std::default;

use async_graphql::{MergedObject, Object};

#[derive(Default)]
pub struct RootMutation();

#[Object]
impl RootMutation {
    async fn user(&self) -> user::UserMutation {
        Default::default()
    }

    async fn topic(&self) -> topic::TopicMutation {
        Default::default()
    }

    async fn reply(&self) -> reply::ReplyMutation {
        let span = tracing::debug_span!("Test");

        Default::default()
    }
}
