use async_graphql::{Context, InputObject, Object, ID};
use surrealdb::sql::Thing;
use tracing::Instrument;

use crate::db::defs::{DBQuery, DBTable, DB};
use crate::db::table::Reply;
use crate::Result;

#[derive(InputObject)]
struct IdOffsetInput {
    id: ID,
    offset: u64,
}

#[derive(Default)]
pub struct ReplyQuery;

#[Object]
impl ReplyQuery {
    async fn get_from_topic(&self, ctx: &Context<'_>, input: IdOffsetInput) -> Result<Vec<Reply>> {
        let db = ctx.data::<DB>()?;

        let topic = Thing::from((DBTable::TOPIC, input.id.as_str()));

        let future = async {
            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_REPLIES_FROM_TOPIC)
                .bind(("topic", &topic))
                .bind(("offset", &input.offset))
                .await?;

            let replies = response.take::<Vec<Reply>>(0)?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(replies)
        };

        let span = tracing::debug_span!("GetFromTopic", id = %input.id.as_str());

        future.instrument(span).await
    }

    async fn get_from_reply(&self, ctx: &Context<'_>, input: IdOffsetInput) -> Result<Vec<Reply>> {
        let db = ctx.data::<DB>()?;

        let reply = Thing::from((DBTable::REPLY, input.id.as_str()));

        let future = async {
            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_REPLIES_FROM_REPLY)
                .bind(("reply", &reply))
                .bind(("offset", &input.offset))
                .await?;

            let replies = response.take::<Vec<Reply>>(1)?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(replies)
        };

        let span = tracing::debug_span!("GetFromReply", id = %input.id.as_str());

        future.instrument(span).await
    }
}
