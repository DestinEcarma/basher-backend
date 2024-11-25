mod reply;
mod topic;

use async_graphql::{Context, MergedObject, Object};

#[derive(Default)]
pub struct RootQuery;

#[Object]
impl RootQuery {
    async fn topic(&self) -> topic::TopicQuery {
        Default::default()
    }

    async fn reply(&self) -> reply::ReplyQuery {
        Default::default()
    }
}
