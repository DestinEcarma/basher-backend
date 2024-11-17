use async_graphql::Object;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct Record {
    id: Thing,
}

impl Record {
    pub fn id(&self) -> &Thing {
        &self.id
    }
}

#[derive(Deserialize)]
pub struct Counter {
    likes: u64,
    shares: u64,
    replies: u64,
    views: Option<u64>,
}

#[Object]
impl Counter {
    async fn likes(&self) -> u64 {
        self.likes
    }

    async fn shares(&self) -> u64 {
        self.shares
    }

    async fn replies(&self) -> u64 {
        self.replies
    }

    async fn views(&self) -> Option<u64> {
        self.views
    }
}
