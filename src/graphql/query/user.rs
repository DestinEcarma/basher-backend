use crate::auth::Auth;
use crate::Result;

use async_graphql::{Context, Object};
use tracing::Instrument;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn auth(&self, ctx: &Context<'_>) -> Result<bool> {
        let future = async {
            match Auth::authenticate(ctx).in_current_span().await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        };

        // Temporary
        let span = tracing::debug_span!("Auth");

        future.instrument(span).await
    }
}
