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

impl From<ClientError> for String {
    fn from(val: ClientError) -> Self {
        if let ClientError::BadRequest(_) = &val {
            tracing::debug!("{val:?}");
        }

        match val {
            ClientError::EmailTaken => "EMAIL_TAKEN".into(),
            ClientError::Unauthorized => "UNAUTHORIZED".into(),
            ClientError::BadRequest(_) => "BAD_REQUEST".into(),
            ClientError::TopicNotFound => "TOPIC_NOT_FOUND".into(),
            ClientError::ReplyNotFound => "REPLY_NOT_FOUND".into(),
            ClientError::EmailNotFound => "EMAIL_NOT_FOUND".into(),
            ClientError::InvalidPassword => "INVALID_PASSWORD".into(),
        }
    }
}

impl From<&ClientError> for StatusCode {
    fn from(val: &ClientError) -> Self {
        match val {
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

impl From<Error> for String {
    fn from(val: Error) -> Self {
        match val {
            Error::Client(client_error) => client_error.into(),
            _ => "Internal Server Error".to_string(),
        }
    }
}

impl From<&Error> for StatusCode {
    fn from(val: &Error) -> Self {
        match val {
            Error::Client(client_error) => client_error.into(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<Error> for async_graphql::Error {
    fn from(val: Error) -> Self {
        let code: StatusCode = (&val).into();

        match &val {
            Error::Client(client_error) => tracing::debug!("ClientError::{client_error:?}"),
            Error::Io(e) => tracing::error!("Error::Io: {e}"),
            Error::Bcrypt(e) => tracing::error!("Error::Bcrypt: {e}"),
            Error::SurrealDB(e) => tracing::error!("Error::SurrealDB: {e}"),
            Error::MissingEnv(e) => tracing::error!("Error::MisingEnv: {e}"),
            Error::JsonWebToken(e) => tracing::error!("Error::JsonWebToken: {e}"),
            Error::AsyncGraphql(e) => tracing::error!("Error::AsyncGraphql: {e:#?}"),
            Error::RecordNotCreated(e) => tracing::error!("Error::RecordNotCreated: {e}"),
            Error::InvalidHeaderValue(e) => tracing::error!("Error::InvalidHeaderValue: {e}"),
        }

        async_graphql::Error::new(val).extend_with(|_, e| e.set("code", code.as_u16()))
    }
}
