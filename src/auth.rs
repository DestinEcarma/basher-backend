use async_graphql::Context;
use chrono::{Duration, Utc};
use cookie::Cookie;
use jsonwebtoken::{errors::ErrorKind, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tower_cookies::Cookies;
use tracing::Instrument;

use crate::{
    config,
    db::{
        defs::{DBQuery, DBTable, DB},
        table::User,
    },
    ClientError, Error, Result,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    sub: String,
}

impl Claims {
    pub fn sub(&self) -> &str {
        &self.sub
    }
}

pub struct Auth;

impl Auth {
    pub const COOKIE_NAME: &'static str = "connect.sid";

    pub async fn generate_jwt(thing: &Thing) -> Result<String> {
        // Temporary
        tracing::debug!(%thing, "Generating JWT");

        let claims = Claims {
            exp: (Utc::now() + Duration::days(30)).timestamp(),
            sub: thing.id.to_string(),
        };

        Ok(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config().JWT_SECRET.as_bytes()),
        )?)
    }

    pub async fn validate_jwt(token: &str) -> Result<Claims> {
        // Temporary
        tracing::debug!("Validating JWT");

        let claims = match jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(config().JWT_SECRET.as_bytes()),
            &Default::default(),
        ) {
            Ok(data) => data.claims,
            Err(err) => {
                // Temporary
                tracing::debug!(%err, "JWT Validation failed");

                match err.kind() {
                    ErrorKind::ExpiredSignature | ErrorKind::InvalidToken => {
                        return Err(Error::Client(ClientError::Unauthorized));
                    }
                    _ => {
                        return Err(Error::JsonWebToken(err));
                    }
                }

                return Err(Error::Client(ClientError::Unauthorized));
            }
        };

        // Temporary
        tracing::debug!(subject = %claims.sub, "JWT Validated");

        Ok(claims)
    }

    pub fn cookie(token: &str, save: bool) -> Cookie {
        let mut cookie = Cookie::build((Self::COOKIE_NAME, token))
            .http_only(true)
            .path("/")
            .build();

        if save {
            cookie.set_max_age(cookie::time::Duration::days(30));
        }

        cookie
    }
}

impl Auth {
    pub async fn authenticate(ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data::<DB>()?;
        let cookies = ctx.data::<Cookies>()?;

        let future = async {
            // Temporary
            tracing::debug!("Authentication");

            let Some(token) = cookies.get(Auth::COOKIE_NAME) else {
                // Temporary
                tracing::debug!("Unauthorized");

                return Err(Error::Client(ClientError::Unauthorized));
            };

            // Temporary
            tracing::debug!("Token found");

            let claims = Auth::validate_jwt(token.value()).await?;

            // Temporary
            tracing::debug!(subject = %claims.sub(), "Authenticating");

            let user = Thing::from((DBTable::USER, claims.sub()));

            let mut response = db.query(DBQuery::SELECT_ID).bind(("thing", user)).await?;

            let Some(user) = response.take::<Option<User>>(0)? else {
                // Temporary
                tracing::debug!("User not found");

                return Err(Error::Client(ClientError::Unauthorized));
            };

            // Temporary
            tracing::debug!("User found");

            Ok(user)
        };

        let span = tracing::debug_span!("Auth");

        future.instrument(span).await
    }
}
