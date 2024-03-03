use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::{
    app_state::AppState,
    router::{healthcheck::HealthCheckRouter, user::UserRouter},
};

pub mod healthcheck;
pub mod user;

pub fn create_main_router(state: AppState) -> Router {
    let client_url = std::env::var("CLIENT_URL").unwrap();
    tracing::info!("CORS allowed for {}", &client_url);

    let allowed_origins = vec![client_url.parse::<HeaderValue>().unwrap()];

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(allowed_origins);

    Router::new()
        .nest("/api/v1/users", UserRouter::create_router(state.clone()))
        .nest("/api/v1/healthcheck", HealthCheckRouter::create_router())
        .with_state(state)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
