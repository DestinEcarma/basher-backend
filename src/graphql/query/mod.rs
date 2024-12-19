mod reply;
mod topic;
mod user;

use async_graphql::Object;

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

    async fn user(&self) -> user::UserQuery {
        Default::default()
    }
}
