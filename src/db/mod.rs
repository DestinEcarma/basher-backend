pub mod defs;
pub mod table;

mod config;

use crate::Result;

use self::config::Config;

pub async fn get_connection() -> Result<defs::DB> {
    let cfg = Config::load_from_env().expect("Could not load configuration from environment");

    let db = surrealdb::engine::any::connect(&cfg.URL)
        .await
        .expect("Could not connect to SurrealDB");

    db.signin(surrealdb::opt::auth::Root {
        username: &cfg.USER,
        password: &cfg.PASS,
    })
    .await
    .expect("Could not sign in to SurrealDB");

    db.use_ns(&cfg.NS).await.expect("Could not use namespace");
    db.use_db(&cfg.DB).await.expect("Could not use database");

    let span = tracing::debug_span!("SurrealDB");
    let _enter = span.enter();

    tracing::debug!("Connected");
    tracing::debug!(namespace = &cfg.NS, "Using");
    tracing::debug!(database = &cfg.DB, "Using");

    Ok(db)
}
