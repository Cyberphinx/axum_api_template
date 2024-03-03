use app_state::{AppState, DB};
use eyre::Result;
use reqwest::Client;
use std::net::IpAddr;
use utilities::token_wrapper::TokenWrapper;

use crate::router::create_main_router;

pub mod app_state;
pub mod middleware;
pub mod models;
pub mod queries;
pub mod router;
pub mod utilities;

pub struct App {
    address: IpAddr,
    port: u16,
    db: DB,
    http_client: Client,
    jwt_secret: String,
}

impl App {
    pub async fn new(
        port: u16,
        database_uri: &str,
        jwt_secret: String,
        environment: &str,
    ) -> Result<Self> {
        let address = if environment.eq("production") {
            // IpV6 for fly.io in production
            IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0])
        } else {
            // IpV4 for local development
            IpAddr::from([127, 0, 0, 1])
        };
        let db = AppState::connect_to_database(database_uri).await?;
        let http_client = AppState::setup_http_client()?;

        tracing_subscriber::fmt::init();

        Ok(Self {
            address,
            port,
            db,
            http_client,
            jwt_secret,
        })
    }

    pub async fn run(&self) -> Result<()> {
        let state = AppState {
            http_client: self.http_client.clone(),
            db: self.db.clone(),
            jwt_secret: TokenWrapper(self.jwt_secret.clone()),
        };

        tracing::info!("Applying pending migrations...");
        AppState::apply_pending_migrations(&state.db)
            .await
            .map_err(|error| {
                tracing::error!("Error applying pending migrations: {error}");
                error
            })?;

        let router = create_main_router(state);
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await?;

        tracing::info!(
            "Server listening on address {} and port {}",
            self.address,
            self.port
        );

        axum::serve(listener, router).await?;

        Ok(())
    }
}
