use async_graphql::{Object, ID};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use surrealdb::sql::{Datetime, Thing};

use crate::db::table::defs::Counter;

#[derive(Deserialize)]
pub struct Topic {
    id: ID,
    title: String,
    tags: Vec<String>,
    content: String,
    counter: Counter,
    activity: DateTime<Utc>,
}

#[Object]
impl Topic {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn tags(&self) -> Vec<&str> {
        self.tags.iter().map(|tag| tag.as_str()).collect()
    }

    async fn content(&self) -> &str {
        &self.content
    }

    async fn activity(&self) -> &DateTime<Utc> {
        &self.activity
    }

    async fn counter(&self) -> &Counter {
        &self.counter
    }
}
