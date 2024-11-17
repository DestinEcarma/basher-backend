use async_graphql::{Context, InputObject, Object, OutputType};
use axum::http::{header, HeaderValue};
use axum::Json;
use cookie::{time::Duration, Cookie};
use std::future::Future;
use tower_cookies::Cookies;
use tracing::{instrument, Instrument};

use crate::auth::Auth;
use crate::db::defs::{DBQuery, DBTable};
use crate::db::{defs::DB, table::User};
use crate::{ClientError, Error, Result};

#[derive(InputObject)]
struct LoginInput {
    #[graphql(validator(email))]
    email: String,

    password: String,
    remember_me: bool,
}

#[derive(InputObject)]
struct SignUpInput {
    #[graphql(validator(email))]
    email: String,

    #[graphql(validator(min_length = 8))]
    password: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let db = ctx.data::<DB>()?;
        let cookies = ctx.data::<Cookies>()?;

        let email = &input.email;
        let password = &input.password;

        let future = async {
            // Temporary
            tracing::debug!("Attempting");

            let mut response = db
                .query(DBQuery::SELECT_ONLY_USER_FROM_EMAIL)
                .bind(("email", email))
                .await?;

            let Some(user) = response.take::<Option<User>>(0)? else {
                return Err(Error::Client(ClientError::EmailNotFound));
            };

            if !(bcrypt::verify(password, user.password())?) {
                return Err(Error::Client(ClientError::InvalidPassword));
            }

            let token = Auth::generate_jwt(user.id()).in_current_span().await?;

            let cookie = Auth::cookie(&token, input.remember_me);

            ctx.append_http_header(header::SET_COOKIE, cookie.to_string());

            // Temporary
            tracing::debug!("Successful");

            Ok("User logged in successfully".to_string())
        };

        // Temporary
        let span = tracing::debug_span!("Login", %email);

        future.instrument(span).await
    }

    async fn sign_up(&self, ctx: &Context<'_>, input: SignUpInput) -> Result<String> {
        let db = ctx.data::<DB>()?;
        let cookies = ctx.data::<Cookies>()?;

        let email = &input.email;
        let password = &input.password;

        let future = async {
            // Temporary
            tracing::debug!("Attempting");

            let mut response = db
                .query(DBQuery::SELECT_ONLY_USER_FROM_EMAIL)
                .bind(("email", email))
                .await?;

            if (response.take::<Option<User>>(0)?).is_some() {
                return Err(Error::Client(ClientError::EmailTaken));
            }

            let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

            let mut response = db
                .query(DBQuery::CREATE_USER)
                .bind(("email", email))
                .bind(("password", password))
                .await?;

            let Some(user) = response.take::<Option<User>>(0)? else {
                return Err(Error::RecordNotCreated(DBTable::USER.to_string()));
            };

            let token = Auth::generate_jwt(user.id()).in_current_span().await?;

            let cookie = Auth::cookie(&token, false);

            ctx.append_http_header(header::SET_COOKIE, cookie.to_string());

            // Temporary
            tracing::debug!("Successful");

            Ok("User created successfully".to_string())
        };

        // Temporary
        let span = tracing::debug_span!("SignUp", %email);

        future.instrument(span).await
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<String> {
        let cookies = ctx.data::<Cookies>()?;

        let future = async {
            let Some(mut cookie) = cookies.get(Auth::COOKIE_NAME) else {
                // Temporary
                tracing::debug!("User already logged out");

                return Ok("User already logged out".to_string());
            };

            cookie.set_max_age(Duration::ZERO);

            ctx.append_http_header(header::SET_COOKIE, cookie.to_string());

            // Temporary
            tracing::debug!("Successful");

            Ok("User logged out successfully".to_string())
        };

        // Temporary
        let span = tracing::debug_span!("Logout");

        future.instrument(span).await
    }
}
