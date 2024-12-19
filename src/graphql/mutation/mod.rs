mod reply;
mod topic;
mod user;

use async_graphql::{MergedObject, Object};
use std::default;

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
        Default::default()
    }
}
