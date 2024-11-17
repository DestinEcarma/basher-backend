use async_graphql::{Object, ID};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use surrealdb::sql::Thing;

use super::defs::Counter;

#[derive(Deserialize)]
pub struct Reply {
    id: ID,
    user_index: u64,
    content: String,
    counter: Counter,
    parent: Option<Parent>,
    activity: DateTime<Utc>,
}

#[Object]
impl Reply {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn user_index(&self) -> u64 {
        self.user_index
    }

    async fn content(&self) -> &str {
        &self.content
    }

    async fn counter(&self) -> &Counter {
        &self.counter
    }

    async fn parent(&self) -> Option<&Parent> {
        self.parent.as_ref()
    }

    async fn activity(&self) -> &DateTime<Utc> {
        &self.activity
    }
}

#[derive(Deserialize)]
struct Parent {
    id: ID,
    user_index: u64,
}

#[Object]
impl Parent {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn user_index(&self) -> u64 {
        self.user_index
    }
}
