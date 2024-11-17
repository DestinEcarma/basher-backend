use async_graphql::Object;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Clone, Deserialize, Serialize)]
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
