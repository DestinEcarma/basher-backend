use async_graphql::Object;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize)]
pub struct Record {
    id: Thing,
}

impl From<Thing> for Record {
    fn from(id: Thing) -> Self {
        Self { id }
    }
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

#[derive(Deserialize)]
pub struct UserStatus {
    identity: u64,
    is_liked: bool,
    is_owner: bool,
    is_shared: bool,
}

#[Object]
impl UserStatus {
    async fn identity(&self) -> u64 {
        self.identity
    }

    async fn is_owner(&self) -> bool {
        self.is_owner
    }

    async fn is_liked(&self) -> bool {
        self.is_liked
    }

    async fn is_shared(&self) -> bool {
        self.is_shared
    }
}
