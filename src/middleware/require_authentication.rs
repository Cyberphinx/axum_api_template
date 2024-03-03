use std::env;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sqlx::{Pool, Postgres};

use crate::{
    models::user::User,
    utilities::{
        app_error::AppError,
        jwt::{validate_admin_token, validate_token},
        token_wrapper::TokenWrapper,
    },
};

pub async fn require_authentication(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    State(pool): State<Pool<Postgres>>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let bearer_token = token.token();

    // return early if token is not valid
    validate_token(&token_secret.0, bearer_token)?;

    // get the user by bearer token
    let user: Option<User> =
        sqlx::query_as!(User, "SELECT * FROM users WHERE token = $1;", bearer_token)
            .fetch_optional(&pool)
            .await?;

    // if token doesn't exist, you are logged out
    if let Some(user) = user {
        // if user is logged in, add the user to the request in an extension
        request.extensions_mut().insert(user);
    } else {
        // if user is not logged in, return early with unauthorized
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ));
    }

    Ok(next.run(request).await)
}

pub async fn require_admin_authentication(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    State(pool): State<Pool<Postgres>>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let bearer_token = token.token();

    // return early if token is not valid
    validate_admin_token(&token_secret.0, bearer_token)?;

    // get the user by bearer token
    let user: Option<User> =
        sqlx::query_as!(User, "SELECT * FROM users WHERE token = $1;", bearer_token)
            .fetch_optional(&pool)
            .await?;

    // if token doesn't exist, you are logged out
    if let Some(user) = user {
        // if user is logged in, add the user to the request in an extension
        request.extensions_mut().insert(user);
    } else {
        // if user is not logged in, return early with unauthorized
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ));
    }

    Ok(next.run(request).await)
}

// machine-to-machine authentication with api key
pub async fn require_m2m<T>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let bearer_token = token.token();
    let api_key = env::var("SPIDERS_API_KEY").unwrap();

    if bearer_token.eq(&api_key) {
        Ok(next.run(request).await)
    } else {
        // if user is not logged in, return early with unauthorized
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ))
    }
}
