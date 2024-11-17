use std::str::FromStr;

use async_graphql::{Context, Object, ID};
use surrealdb::sql::Thing;
use tracing::Instrument;

use crate::db::defs::{DBQuery, DBTable, DB};
use crate::db::table::Topic;
use crate::Result;

#[derive(Default)]
pub struct TopicQuery;

#[Object]
impl TopicQuery {
    async fn get(&self, ctx: &Context<'_>, offset: u64) -> Result<Vec<Topic>> {
        let db = ctx.data::<DB>()?;

        let future = async {
            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_TOPICS)
                .bind(("offset", offset))
                .await?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(response.take::<Vec<Topic>>(0)?)
        };

        // Temporary
        let span = tracing::debug_span!("GetTopics", %offset);

        future.instrument(span).await
    }

    async fn get_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Topic>> {
        let db = ctx.data::<DB>()?;

        let topic = Thing::from((DBTable::TOPIC, id.as_str()));

        let future = async {
            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_ONLY_TOPIC)
                .bind(("topic", &topic))
                .await?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(response.take::<Option<Topic>>(0)?)
        };

        // Temporary
        let span = tracing::debug_span!("GetTopicById", id = %id.as_str());

        future.instrument(span).await
    }
}
