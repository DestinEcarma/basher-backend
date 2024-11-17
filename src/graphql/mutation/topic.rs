use async_graphql::{Context, InputObject, Object, ID};
use serde::Serialize;
use surrealdb::sql::Thing;
use tower_cookies::Cookies;
use tracing::Instrument;

use crate::db::defs::{DBQuery, DBTable};
use crate::db::table::User;
use crate::graphql::query;
use crate::{auth::Auth, db::defs::DB};
use crate::{ClientError, Error, Result};

#[derive(InputObject, Serialize)]
struct Tag {
    name: String,
}

#[derive(InputObject)]
struct CreateTopicInput {
    title: String,
    tags: Vec<Tag>,
    content: String,
}

#[derive(Default)]
pub struct TopicMutation;

#[Object]
impl TopicMutation {
    async fn create(&self, ctx: &Context<'_>, input: CreateTopicInput) -> Result<ID> {
        let db = ctx.data::<DB>()?;

        let future = async {
            let user = Auth::authenticate(ctx).in_current_span().await?;

            let future = async {
                // Temporary
                tracing::debug!("Creating topic");

                let mut response = db
                    .query(DBQuery::CREATE_TOPIC)
                    .bind(("user", user.id()))
                    .bind(("tags", &input.tags))
                    .bind(("title", &input.title))
                    .bind(("content", &input.content))
                    .await?;

                let Some(id) = response.take::<Option<ID>>(3)? else {
                    // Temporary
                    tracing::debug!("Topic not created");

                    return Err(Error::RecordNotCreated(DBTable::TOPIC.to_string()));
                };

                // Temporary
                tracing::debug!(id = %id.as_str(), "Topic created");

                Ok(id)
            };

            let span = tracing::debug_span!("Create", user = %user.id());

            future.instrument(span).await
        };

        let span = tracing::debug_span!("Topic");

        future.instrument(span).await
    }
}
