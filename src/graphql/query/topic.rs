use crate::auth::Auth;
use crate::db::defs::{DBQuery, DBTable, SharedDB};
use crate::db::table::Topic;
use crate::Result;

use async_graphql::{Context, InputObject, Object, ID};
use std::collections::HashSet;
use surrealdb::sql::Thing;
use tracing::Instrument;

#[derive(InputObject, Clone)]
struct SearchTopicInput {
    query: String,
    tags: String,
    offset: u64,
}

#[derive(Default)]
pub struct TopicQuery;

#[Object]
impl TopicQuery {
    async fn get(&self, ctx: &Context<'_>, offset: u64) -> Result<Vec<Topic>> {
        let db = ctx.data::<SharedDB>()?;

        let future = async {
            let user = Auth::authenticate(ctx)
                .in_current_span()
                .await
                .unwrap_or_default();

            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_TOPICS)
                .bind(("offset", offset))
                .bind(("user", user.id().to_owned()))
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
        let db = ctx.data::<SharedDB>()?;

        let topic = Thing::from((DBTable::TOPIC, id.as_str()));

        let future = async {
            let user = Auth::authenticate(ctx)
                .in_current_span()
                .await
                .unwrap_or_default();

            // Temporary
            tracing::debug!("Retrieving data");

            let mut response = db
                .query(DBQuery::SELECT_ONLY_TOPIC)
                .bind(("topic", topic.to_owned()))
                .bind(("user", user.id().to_owned()))
                .await?;

            // Temporary
            tracing::debug!("Data retrieved");

            Ok(response.take::<Option<Topic>>(0)?)
        };

        // Temporary
        let span = tracing::debug_span!("GetTopicById", id = %id.as_str());

        future.instrument(span).await
    }

    async fn search(&self, ctx: &Context<'_>, input: SearchTopicInput) -> Result<Vec<Topic>> {
        let db = ctx.data::<SharedDB>()?;

        let input_clone = input.clone();

        let future = async {
            // Temporary
            tracing::debug!("Searching data");

            let mut response = db
                .query(DBQuery::SELECT_TOPICS_FROM_QUERY)
                .bind(("offset", input.offset))
                .bind(("query", input.query.to_owned()))
                .bind((
                    "tags",
                    input
                        .tags
                        .split_whitespace()
                        .into_iter()
                        .map(|tag| tag.trim_start_matches('#').to_lowercase())
                        .collect::<HashSet<String>>()
                        .into_iter()
                        .collect::<Vec<String>>(),
                ))
                .await?;

            // Temporary
            tracing::debug!("Data searched");

            Ok(response.take::<Vec<Topic>>(0)?)
        };

        // Temporary
        let span = tracing::debug_span!("SearchTopics", %input_clone.query);

        future.instrument(span).await
    }
}
