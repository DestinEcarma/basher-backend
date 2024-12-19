use crate::auth::Auth;
use crate::db::defs::{DBQuery, DBTable, SharedDB};
use crate::db::table::Reply;
use crate::Result;

use async_graphql::{Context, InputObject, Object, ID};
use surrealdb::sql::Thing;
use tracing::Instrument;

#[derive(InputObject, Clone)]
struct GetRepliesInput {
    id: ID,
    offset: u64,
}

#[derive(InputObject, Clone)]
struct GetRepliesFromReplyInput {
    topic: ID,
    reply: ID,
    offset: u64,
}

#[derive(InputObject, Clone)]
struct GetReplyInput {
    topic: ID,
    reply: ID,
}

#[derive(Default)]
pub struct ReplyQuery;

#[Object]
impl ReplyQuery {
    async fn get_reply(&self, ctx: &Context<'_>, input: GetReplyInput) -> Result<Option<Reply>> {
        let db = ctx.data::<SharedDB>()?;

        let reply = Thing::from((DBTable::REPLY, input.reply.as_str()));
        let topic = Thing::from((DBTable::TOPIC, input.topic.as_str()));

        let future = async {
            let user = Auth::authenticate(ctx)
                .in_current_span()
                .await
                .unwrap_or_default();

            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_ONLY_REPLY)
                .bind(("reply", reply.to_owned()))
                .bind(("topic", topic.to_owned()))
                .bind(("user", user.id().to_owned()))
                .await?;

            let reply = response.take::<Option<Reply>>(0)?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(reply)
        };

        let span = tracing::debug_span!("GetReply", id = %input.reply.as_str());

        future.instrument(span).await
    }

    async fn get_from_topic(
        &self,
        ctx: &Context<'_>,
        input: GetRepliesInput,
    ) -> Result<Vec<Reply>> {
        let db = ctx.data::<SharedDB>()?;

        let topic = Thing::from((DBTable::TOPIC, input.id.as_str()));

        let future = async {
            let user = Auth::authenticate(ctx)
                .in_current_span()
                .await
                .unwrap_or_default();

            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_REPLIES_FROM_TOPIC)
                .bind(("topic", topic.to_owned()))
                .bind(("user", user.id().to_owned()))
                .bind(("offset", input.offset.to_owned()))
                .await?;

            let replies = response.take::<Vec<Reply>>(0)?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(replies)
        };

        let span = tracing::debug_span!("GetFromTopic", id = %input.id.as_str());

        future.instrument(span).await
    }

    async fn get_from_reply(
        &self,
        ctx: &Context<'_>,
        input: GetRepliesFromReplyInput,
    ) -> Result<Vec<Reply>> {
        let db = ctx.data::<SharedDB>()?;

        let topic = Thing::from((DBTable::TOPIC, input.topic.as_str()));
        let reply = Thing::from((DBTable::REPLY, input.reply.as_str()));

        let future = async {
            let user = Auth::authenticate(ctx)
                .in_current_span()
                .await
                .unwrap_or_default();

            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_REPLIES_FROM_REPLY)
                .bind(("topic", topic.to_owned()))
                .bind(("reply", reply.to_owned()))
                .bind(("user", user.id().to_owned()))
                .bind(("offset", input.offset.to_owned()))
                .await?;

            let replies = response.take::<Vec<Reply>>(0)?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(replies)
        };

        let span = tracing::debug_span!("GetRepliesFromReply", id = %input.reply.as_str());

        future.instrument(span).await
    }
}
