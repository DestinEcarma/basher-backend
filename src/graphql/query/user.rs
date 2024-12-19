use crate::auth::Auth;
use crate::db::defs::SharedDB;
use crate::Result;

use async_graphql::{Context, Object};
use tracing::{error, Instrument};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn auth(&self, ctx: &Context<'_>) -> Result<bool> {
        let db = ctx.data::<SharedDB>()?;

        let future = async {
            match Auth::authenticate(ctx).in_current_span().await {
                Ok(_) => Ok(true),
                Err(e) => Ok(false),
            }
        };

        // Temporary
        let span = tracing::debug_span!("Auth");

        future.instrument(span).await
    }
}
