use crate::db::defs::{DBQuery, DBTable};
use crate::db::table::Record;
use crate::graphql::defs::validate_topic;
use crate::sse::defs::{ReplyData, SharedReplyChannels, SharedTopicTX, TopicData};
use crate::{auth::Auth, db::defs::SharedDB};
use crate::{ClientError, Error, Result};

use async_graphql::{Context, InputObject, Object, ID};
use std::collections::HashSet;
use surrealdb::sql::Thing;
use tracing::Instrument;

#[derive(InputObject)]
struct CreateTopicInput {
    title: String,
    tags: String,
    content: String,
}

#[derive(InputObject)]
struct UpdateTopicInput {
    id: ID,
    title: String,
    tags: String,
    content: String,
}

#[derive(Default)]
pub struct TopicMutation;

#[Object]
impl TopicMutation {
    async fn create(&self, ctx: &Context<'_>, input: CreateTopicInput) -> Result<ID> {
        let db = ctx.data::<SharedDB>()?;
        let tx = ctx.data::<SharedTopicTX>()?;

        let future = async {
            let user = Auth::authenticate(ctx)
                .in_current_span()
                .await
                .unwrap_or_default();

            let user_clone = user.clone();

            let future = async {
                // Temporary
                tracing::debug!("Creating topic");

                let mut response = db
                    .query(DBQuery::CREATE_TOPIC)
                    .bind(("user", user.id().to_owned()))
                    .bind(("title", input.title.to_owned()))
                    .bind(("content", input.content.to_owned()))
                    .bind((
                        "tags",
                        input
                            .tags
                            .split_whitespace()
                            .into_iter()
                            .map(|tag| tag.trim_start_matches('#').to_lowercase())
                            .collect::<HashSet<String>>()
                            .into_iter()
                            .map(|tag| Record::from(Thing::from((DBTable::TAG, tag.as_str()))))
                            .collect::<Vec<Record>>(),
                    ))
                    .await?;

                let Some(id) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Topic not created");

                    return Err(Error::RecordNotCreated(DBTable::TOPIC.to_string()));
                };

                // Temporary
                tracing::debug!(id = %id.as_str(), "Topic created");
                tracing::debug!(path = "/sse/topic", "Sending to subscribers");

                let _ = tx.send(TopicData::new(id.clone()));

                Ok(id)
            };

            let span = tracing::debug_span!("Create", user = %user_clone.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Topic");

        future.instrument(span).await
    }

    async fn update(&self, ctx: &Context<'_>, input: UpdateTopicInput) -> Result<&str> {
        let db = ctx.data::<SharedDB>()?;
        let channels = ctx.data::<SharedReplyChannels>()?;

        let future = async {
            let user = Auth::authenticate(ctx).in_current_span().await?;
            let topic = validate_topic(db, &input.id).await?;

            let future = async {
                // Temporary
                tracing::debug!("Updating topic");

                let mut response = db
                    .query(DBQuery::UPDATE_TOPIC)
                    .bind(("user", user.id().to_owned()))
                    .bind(("topic", topic.id().to_owned()))
                    .bind(("title", input.title.to_owned()))
                    .bind(("content", input.content.to_owned()))
                    .bind((
                        "tags",
                        input
                            .tags
                            .split_whitespace()
                            .into_iter()
                            .map(|tag| tag.trim_start_matches('#').to_lowercase())
                            .collect::<HashSet<String>>()
                            .into_iter()
                            .map(|tag| Record::from(Thing::from((DBTable::TAG, tag.as_str()))))
                            .collect::<Vec<Record>>(),
                    ))
                    .await?;

                let Some(_) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Topic not updated");

                    return Err(Error::Client(ClientError::Unauthorized));
                };

                // Temporary
                tracing::debug!("Topic updated");
                tracing::debug!(
                    path = format!("/sse/topic/{}", input.id.as_str()),
                    "Sending to subscribers"
                );

                let channels = channels.lock().await;

                if let Some(tx) = channels.get(input.id.as_str()) {
                    let _ = tx.send(ReplyData::new(input.id.clone(), "Updated", "Topic"));
                }

                Ok("Topic updated successfully")
            };

            let span = tracing::debug_span!("Update", user = %user.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Topic");

        future.instrument(span).await
    }

    async fn like(&self, ctx: &Context<'_>, id: ID) -> Result<&str> {
        let db = ctx.data::<SharedDB>()?;

        let future = async {
            let user = Auth::authenticate(ctx).in_current_span().await?;

            let future = async {
                // Temporary
                tracing::debug!("Liking topic");

                let mut response = db
                    .query(DBQuery::LIKE_POST)
                    .bind(("user", user.id().to_owned()))
                    .bind(("post", Thing::from((DBTable::TOPIC, id.as_str()))))
                    .await?;

                let Some(_) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Topic not liked");

                    return Err(Error::Client(ClientError::Unauthorized));
                };

                // Temporary
                tracing::debug!("Topic liked");
                //tracing::debug!(
                //    path = format!("/sse/topic/{}", id.as_str()),
                //    "Sending to subscribers"
                //);
                //
                //let channels = channels.lock().await;
                //
                //if let Some(tx) = channels.get(id.as_str()) {
                //    tx.send(ReplyData::new(id.clone(), "Liked", "Topic"));
                //}

                Ok("Topic liked successfully")
            };

            let span = tracing::debug_span!("Like", user = %user.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Topic", id = %id.as_str());

        future.instrument(span).await
    }

    async fn share(&self, ctx: &Context<'_>, id: ID) -> Result<&str> {
        let db = ctx.data::<SharedDB>()?;

        let future = async {
            let user = Auth::authenticate(ctx).in_current_span().await?;

            let future = async {
                // Temporary
                tracing::debug!("Sharing topic");

                let mut response = db
                    .query(DBQuery::SHARE_POST)
                    .bind(("user", user.id().to_owned()))
                    .bind(("post", Thing::from((DBTable::TOPIC, id.as_str()))))
                    .await?;

                let Some(_) = response.take::<Option<ID>>(0)? else {
                    // Temporary
                    tracing::debug!("Topic not shared");

                    return Err(Error::Client(ClientError::Unauthorized));
                };

                // Temporary
                tracing::debug!("Topic shared");
                //tracing::debug!(
                //    path = format!("/sse/topic/{}", id.as_str()),
                //    "Sending to subscribers"
                //);
                //
                //let channels = channels.lock().await;
                //
                //if let Some(tx) = channels.get(id.as_str()) {
                //    tx.send(ReplyData::new(id.clone(), "Update", "Topic"));
                //}

                Ok("Topic shared successfully")
            };

            let span = tracing::debug_span!("Share", user = %user.id().id.to_raw());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Topic", id = %id.as_str());

        future.instrument(span).await
    }
}
