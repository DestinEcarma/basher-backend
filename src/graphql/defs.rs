use crate::{
    db::{
        defs::{DBQuery, DBTable, SharedDB},
        table::Record,
    },
    ClientError, Error, Result,
};

use super::{RootMutation, RootQuery};

use async_graphql::{EmptySubscription, InputObject, Object};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use tracing::Instrument;

pub type ApiSchema = async_graphql::Schema<RootQuery, RootMutation, EmptySubscription>;

#[derive(InputObject, Serialize, Clone, Debug)]
pub struct Tag {
    name: String,
}

impl From<&str> for Tag {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Time {
    created_at: Datetime,
    updated_at: Datetime,
}

#[Object]
impl Time {
    async fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    async fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}

pub async fn validate_topic(db: &SharedDB, id: &str) -> Result<Record> {
    let topic = Thing::from((DBTable::TOPIC, id));

    let topic_clone = topic.clone();

    let future = async {
        let mut response = db
            .query(DBQuery::SELECT_ID)
            .bind(("thing", topic.to_owned()))
            .await?;

        let Some(record) = response.take::<Option<Record>>(0)? else {
            // Temporary
            tracing::debug!("Topic not found");

            return Err(Error::Client(ClientError::TopicNotFound));
        };

        Ok(record)
    };

    let span = tracing::debug_span!("Validate", %topic_clone);

    future.instrument(span).await
}
