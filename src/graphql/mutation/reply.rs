use crate::auth::Auth;
use crate::db::defs::{DBQuery, DBTable, SharedDB};
use crate::db::table::Record;
use crate::graphql::defs::validate_topic;
use crate::sse::defs::{ReplyData, SharedReplyChannels};
use crate::{ClientError, Error, Result};

use async_graphql::{Context, InputObject, Object, ID};
use surrealdb::sql::Thing;
use tracing::Instrument;

#[derive(InputObject, Clone)]
struct CreateReplyInput {
    topic: ID,
    content: String,
    parent: Option<ID>,
}

#[derive(InputObject, Clone)]
struct UpdateReplyInput {
    topic: ID,
    reply: ID,
    content: String,
}

#[derive(Default)]
pub struct ReplyMutation;

#[Object]
impl ReplyMutation {
    async fn create(&self, ctx: &Context<'_>, input: CreateReplyInput) -> Result<&str> {
        let db = ctx.data::<SharedDB>()?;
        let channels = ctx.data::<SharedReplyChannels>()?;

        let input_clone = input.clone();

        let future = async move {
            let user = Auth::authenticate(ctx).in_current_span().await?;
            let topic = validate_topic(db, &input.topic).await?;

            let parent = match input.parent {
                Some(parent) => Some(Self::validate_reply(db, &parent).await?),
                None => None,
            };

            let parent = parent.as_ref().map(|p| p.id().to_owned());

            let future = async {
                // Temporary
                tracing::debug!("Creating reply");

                let mut response = db
                    .query(DBQuery::CREATE_REPLY)
                    .bind(("content", input.content))
                    .bind(("parent", parent.to_owned()))
                    .bind(("user", user.id().to_owned()))
                    .bind(("topic", topic.id().to_owned()))
                    .await?;

                let Some(id) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Reply not created");

                    return Err(Error::RecordNotCreated(DBTable::REPLY.to_string()));
                };

                // Temporary
                tracing::debug!(id = %id.as_str(), "Reply created");
                tracing::debug!(
                    path = format!("/sse/topic/{}", input.topic.as_str()),
                    "Sending to subscribers"
                );

                let channels = channels.lock().await;

                if let Some(tx) = channels.get(input.topic.as_str()) {
                    let _ = tx.send(ReplyData::new(id.clone(), "Created", "Reply"));
                }

                Ok("Reply created successfully")
            };

            let span = tracing::debug_span!("Create", user = %user.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Reply", topic = %input_clone.topic.as_str());

        future.instrument(span).await
    }

    async fn update(&self, ctx: &Context<'_>, input: UpdateReplyInput) -> Result<&str> {
        let db = ctx.data::<SharedDB>()?;
        let channels = ctx.data::<SharedReplyChannels>()?;

        let input_clone = input.clone();

        let future = async move {
            let user = Auth::authenticate(ctx).in_current_span().await?;

            let future = async {
                // Temporary
                tracing::debug!("Updating reply");

                let mut response = db
                    .query(DBQuery::UPDATE_REPLY)
                    .bind(("content", input.content))
                    .bind(("user", user.id().to_owned()))
                    .bind(("reply", input.reply.to_owned()))
                    .await?;

                let Some(id) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Reply not updated");

                    return Err(Error::Client(ClientError::Unauthorized));
                };

                // Temporary
                tracing::debug!(id = %id.as_str(), "Reply updated");
                tracing::debug!(
                    path = format!("/sse/topic/{}", input.topic.as_str()),
                    "Sending to subscribers"
                );

                let channels = channels.lock().await;

                if let Some(tx) = channels.get(input.topic.as_str()) {
                    let _ = tx.send(ReplyData::new(id.clone(), "Updated", "Reply"));
                }

                Ok("Reply updated successfully")
            };

            let span = tracing::debug_span!("Update", user = %user.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Reply", topic = %input_clone.topic.as_str());

        future.instrument(span).await
    }

    async fn like(&self, ctx: &Context<'_>, id: ID) -> Result<&str> {
        let db = ctx.data::<SharedDB>()?;

        let future = async {
            let user = Auth::authenticate(ctx).in_current_span().await?;

            let future = async {
                // Temporary
                tracing::debug!("Liking reply");

                let mut response = db
                    .query(DBQuery::LIKE_POST)
                    .bind(("user", user.id().to_owned()))
                    .bind(("post", Thing::from((DBTable::REPLY, id.as_str()))))
                    .await?;

                let Some(_) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Reply not liked");

                    return Err(Error::Client(ClientError::Unauthorized));
                };

                // Temporary
                tracing::debug!("Reply liked");
                //tracing::debug!(
                //    path = format!("/sse/topic/{}", id.as_str()),
                //    "Sending to subscribers"
                //);
                //
                //let channels = channels.lock().await;
                //
                //if let Some(tx) = channels.get(id.as_str()) {
                //    tx.send(ReplyData::new(id.clone(), "Liked", "Reply"));
                //}

                Ok("Reply liked successfully")
            };

            let span = tracing::debug_span!("Like", user = %user.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Reply", id = %id.as_str());

        future.instrument(span).await
    }

    async fn share(&self, ctx: &Context<'_>, id: ID) -> Result<&str> {
        let db = ctx.data::<SharedDB>()?;

        let future = async {
            let user = Auth::authenticate(ctx).in_current_span().await?;

            let future = async {
                // Temporary
                tracing::debug!("Sharing reply");

                let mut response = db
                    .query(DBQuery::SHARE_POST)
                    .bind(("user", user.id().to_owned()))
                    .bind(("post", Thing::from((DBTable::REPLY, id.as_str()))))
                    .await?;

                let Some(_) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Reply not shared");

                    return Err(Error::Client(ClientError::Unauthorized));
                };

                // Temporary
                tracing::debug!("Reply shared");
                //tracing::debug!(
                //    path = format!("/sse/topic/{}", id.as_str()),
                //    "Sending to subscribers"
                //);
                //
                //let channels = channels.lock().await;
                //
                //if let Some(tx) = channels.get(id.as_str()) {
                //    tx.send(ReplyData::new(id.clone(), "Shared", "Reply"));
                //}

                Ok("Reply shared successfully")
            };

            let span = tracing::debug_span!("Share", user = %user.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Reply", id = %id.as_str());

        future.instrument(span).await
    }
}

impl ReplyMutation {
    async fn validate_reply(db: &SharedDB, id: &str) -> Result<Record> {
        let reply = Thing::from((DBTable::REPLY, id));

        let reply_clone = reply.clone();

        let future = async {
            let mut response = db
                .query(DBQuery::SELECT_ID)
                .bind(("thing", reply.to_owned()))
                .await?;

            let Some(record) = response.take::<Option<Record>>(0)? else {
                // Temporary
                tracing::debug!("Reply not found");

                return Err(Error::Client(ClientError::ReplyNotFound));
            };

            Ok(record)
        };

        let span = tracing::debug_span!("Validate", %reply_clone);

        future.instrument(span).await
    }
}
