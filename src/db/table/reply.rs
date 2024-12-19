use super::defs::{Counter, UserStatus};

use async_graphql::{Object, ID};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Reply {
    id: ID,
    content: String,
    counter: Counter,
    parent: Option<Parent>,
    activity: DateTime<Utc>,
    user_status: UserStatus,
}

#[Object]
impl Reply {
    async fn id(&self) -> &ID {
        &self.id
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

    async fn user_status(&self) -> &UserStatus {
        &self.user_status
    }
}

#[derive(Deserialize, Clone)]
struct Parent {
    id: ID,
    user_identity: u64,
}

#[Object]
impl Parent {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn user_identity(&self) -> u64 {
        self.user_identity
    }
}
