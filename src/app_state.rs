use axum::extract::FromRef;
use eyre::Result;
use reqwest::Client;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};

use crate::utilities::token_wrapper::TokenWrapper;

/// Database Connection App State Type
pub type DB = Pool<Postgres>;

/// Project AppState struct to provide cross-cutting values across the app
#[derive(Clone, FromRef)]
pub struct AppState {
    pub http_client: Client,
    pub db: DB,
    pub jwt_secret: TokenWrapper,
}

// Define the migrator with the path to your migrations
static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

impl AppState {
    /// Database Connection via sqlx postgres
    pub async fn connect_to_database(database_uri: &str) -> Result<DB> {
        Ok(PgPoolOptions::new()
            .max_connections(5)
            .connect(database_uri)
            .await?)
    }

    /// Setting up reqwest http client
    pub fn setup_http_client() -> Result<Client> {
        Ok(Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
        .build()?)
    }

    /// Apply migrations
    pub async fn apply_pending_migrations(db: &DB) -> Result<()> {
        Ok(MIGRATOR.run(db).await?)
    }
}
