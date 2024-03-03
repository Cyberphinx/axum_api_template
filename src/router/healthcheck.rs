use axum::{routing::get, Router};

use crate::app_state::AppState;

pub struct HealthCheckRouter;

impl HealthCheckRouter {
    pub fn create_router() -> Router<AppState> {
        Router::new().route("/", get(Self::health_check))
    }

    pub async fn health_check() -> &'static str {
        "I am healthy!!!"
    }

    pub async fn health_check_auth() -> &'static str {
        "I am healthy from authenticated user!!!"
    }

    pub async fn health_check_admin() -> &'static str {
        "I am healthy from admin!!!"
    }

    pub async fn health_check_m2m() -> &'static str {
        "I am healthy from authenticated machine to machine route!!!"
    }
}
