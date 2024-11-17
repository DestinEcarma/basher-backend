use async_graphql::{Context, InputObject, Object, ID};
use surrealdb::sql::Thing;
use tracing::{span, Instrument};

use crate::auth::Auth;
use crate::db::defs::{DBQuery, DBTable, DB};
use crate::db::table::Record;
use crate::{ClientError, Error, Result};

#[derive(InputObject)]
struct CreateReplyInput {
    topic: ID,
    content: String,
    parent: Option<String>,
}

#[derive(Default)]
pub struct ReplyMutation;

#[Object]
impl ReplyMutation {
    async fn create(&self, ctx: &Context<'_>, input: CreateReplyInput) -> Result<ID> {
        let db = ctx.data::<DB>()?;

        let future = async {
            let user = Auth::authenticate(ctx).in_current_span().await?;
            let topic = Self::validate_topic(db, &input.topic).await?;

            let parent = match input.parent {
                Some(parent) => Some(Self::validate_reply(db, &parent).await?),
                None => None,
            };

            let future = async {
                // Temporary
                tracing::debug!("Creating reply");

                let mut response = db
                    .query(DBQuery::CREATE_REPLY)
                    .bind(("user", user.id()))
                    .bind(("topic", topic.id()))
                    .bind(("content", &input.content))
                    .bind(("parent", parent.as_ref().map(|p| p.id())))
                    .await?;

                let Some(id) = response.take::<Option<ID>>(4)? else {
                    // Temporary
                    tracing::debug!("Reply not created");

                    return Err(Error::RecordNotCreated(DBTable::REPLY.to_string()));
                };

                // Temporary
                tracing::debug!(id = %id.as_str(), "Reply created");

                Ok(id)
            };

            let span = tracing::debug_span!("Create", user = %user.id());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Reply", topic = %input.topic.as_str());

        future.instrument(span).await
    }
}

impl ReplyMutation {
    async fn validate_topic(db: &DB, id: &str) -> Result<Record> {
        let topic = Thing::from((DBTable::TOPIC, id));

        let future = async {
            let mut response = db.query(DBQuery::SELECT_ID).bind(("thing", &topic)).await?;

            let Some(record) = response.take::<Option<Record>>(0)? else {
                // Temporary
                tracing::debug!("Topic not found");

                return Err(Error::Client(ClientError::TopicNotFound));
            };

            Ok(record)
        };

        let span = tracing::debug_span!("Validate", %topic);

        future.instrument(span).await
    }

    async fn validate_reply(db: &DB, id: &str) -> Result<Record> {
        let reply = Thing::from((DBTable::REPLY, id));

        let future = async {
            let mut response = db.query(DBQuery::SELECT_ID).bind(("thing", &reply)).await?;

            let Some(record) = response.take::<Option<Record>>(0)? else {
                // Temporary
                tracing::debug!("Reply not found");

                return Err(Error::Client(ClientError::ReplyNotFound));
            };

            Ok(record)
        };

        let span = tracing::debug_span!("Validate", %reply);

        future.instrument(span).await
    }
}
