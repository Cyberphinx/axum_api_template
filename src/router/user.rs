use axum::{extract::State, http::StatusCode, middleware, routing::post, Extension, Json, Router};
use eyre::Result;

use crate::{
    app_state::{AppState, DB},
    middleware::require_authentication::{require_admin_authentication, require_authentication},
    models::user::User,
    queries::users::{
        RequestCreateUser, RequestLoginUser, ResponseDataUser, ResponseLogoutUser, UserQueries,
    },
    utilities::{app_error::AppError, token_wrapper::TokenWrapper},
};

pub struct UserRouter;

impl UserRouter {
    pub fn create_router(state: AppState) -> Router<AppState> {
        Router::new()
            .route("/admin/signup", post(Self::create_admin_user))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                require_admin_authentication,
            ))
            .route("/logout", post(Self::logout))
            .layer(middleware::from_fn_with_state(
                state,
                require_authentication,
            ))
            .route("/signup", post(Self::create_user))
            .route("/login", post(Self::login))
    }

    pub async fn create_user(
        State(db): State<DB>,
        State(jwt_secret): State<TokenWrapper>,
        Json(request_user): Json<RequestCreateUser>,
    ) -> Result<Json<ResponseDataUser>, AppError> {
        let response = UserQueries::create_user(db, jwt_secret, request_user, "user".to_string())
            .await
            .map_err(|error| {
                tracing::error!("Problem creating user: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Problem creating user")
            })?;
        Ok(response)
    }

    pub async fn create_admin_user(
        State(db): State<DB>,
        State(jwt_secret): State<TokenWrapper>,
        Json(request_user): Json<RequestCreateUser>,
    ) -> Result<Json<ResponseDataUser>, AppError> {
        let response = UserQueries::create_user(db, jwt_secret, request_user, "admin".to_string())
            .await
            .map_err(|error| {
                tracing::error!("Problem creating admin user: {}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Problem creating admin user",
                )
            })?;
        Ok(response)
    }

    pub async fn login(
        State(db): State<DB>,
        State(jwt_secret): State<TokenWrapper>,
        Json(request_user): Json<RequestLoginUser>,
    ) -> Result<Json<ResponseDataUser>, AppError> {
        let response = UserQueries::login(db, jwt_secret, request_user)
            .await
            .map_err(|error| {
                tracing::error!("Problem logging in the user: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Problem logging in")
            })?;
        Ok(response)
    }

    pub async fn logout(
        Extension(user): Extension<User>,
        State(db): State<DB>,
    ) -> Result<Json<ResponseLogoutUser>, AppError> {
        let response = UserQueries::logout(db, user).await.map_err(|error| {
            tracing::error!("Problem logging out the user: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Problem logging out")
        })?;
        Ok(response)
    }
}
