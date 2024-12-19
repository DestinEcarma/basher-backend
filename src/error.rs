use async_graphql::ErrorExtensions;
use axum::http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum ClientError {
    BadRequest(String),

    // Sign Up Errors
    EmailTaken,

    // Login Errors
    EmailNotFound,
    InvalidPassword,

    // Topic Errors
    TopicNotFound,

    // Reply Errors
    ReplyNotFound,

    // Auth Errors
    Unauthorized,
}

impl Into<String> for ClientError {
    fn into(self) -> String {
        if let Self::BadRequest(_) = &self {
            tracing::debug!("{self:?}");
        }

        match self {
            Self::EmailTaken => "EMAIL_TAKEN".into(),
            Self::Unauthorized => "UNAUTHORIZED".into(),
            Self::BadRequest(_) => "BAD_REQUEST".into(),
            Self::TopicNotFound => "TOPIC_NOT_FOUND".into(),
            Self::ReplyNotFound => "REPLY_NOT_FOUND".into(),
            Self::EmailNotFound => "EMAIL_NOT_FOUND".into(),
            Self::InvalidPassword => "INVALID_PASSWORD".into(),
        }
    }
}

impl Into<StatusCode> for &ClientError {
    fn into(self) -> StatusCode {
        match self {
            ClientError::EmailTaken => StatusCode::CONFLICT,
            ClientError::EmailNotFound => StatusCode::NOT_FOUND,
            ClientError::TopicNotFound => StatusCode::NOT_FOUND,
            ClientError::ReplyNotFound => StatusCode::NOT_FOUND,
            ClientError::Unauthorized => StatusCode::UNAUTHORIZED,
            ClientError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ClientError::InvalidPassword => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum Error {
    // Common Errors
    #[from]
    Io(std::io::Error),
    #[from]
    SurrealDB(surrealdb::Error),
    #[from]
    Bcrypt(bcrypt::BcryptError),
    #[from]
    AsyncGraphql(async_graphql::Error),
    #[from]
    JsonWebToken(jsonwebtoken::errors::Error),
    #[from]
    InvalidHeaderValue(axum::http::header::InvalidHeaderValue),

    // Unique Errors
    MissingEnv(String),
    RecordNotCreated(String),

    // Client Errors
    #[from]
    Client(ClientError),
}

impl Into<String> for Error {
    fn into(self) -> String {
        match self {
            Self::Client(client_error) => client_error.into(),
            _ => "Internal Server Error".to_string(),
        }
    }
}

impl Into<StatusCode> for &Error {
    fn into(self) -> StatusCode {
        match self {
            Error::Client(client_error) => client_error.into(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Into<async_graphql::Error> for Error {
    fn into(self) -> async_graphql::Error {
        let code: StatusCode = (&self).into();

        match &self {
            Self::Client(client_error) => tracing::debug!("ClientError::{client_error:?}"),
            Self::Io(e) => tracing::error!("Error::Io: {e}"),
            Self::Bcrypt(e) => tracing::error!("Error::Bcrypt: {e}"),
            Self::SurrealDB(e) => tracing::error!("Error::SurrealDB: {e}"),
            Self::MissingEnv(e) => tracing::error!("Error::MisingEnv: {e}"),
            Self::JsonWebToken(e) => tracing::error!("Error::JsonWebToken: {e}"),
            Self::AsyncGraphql(e) => tracing::error!("Error::AsyncGraphql: {e:#?}"),
            Self::RecordNotCreated(e) => tracing::error!("Error::RecordNotCreated: {e}"),
            Self::InvalidHeaderValue(e) => tracing::error!("Error::InvalidHeaderValue: {e}"),
        }

        async_graphql::Error::new(self).extend_with(|_, e| e.set("code", code.as_u16()))
    }
}
