use axum::{http::StatusCode, Json};
use eyre::Result;

use crate::{
    app_state::DB,
    models::user::User,
    utilities::{
        app_error::AppError, hash::verify_password, jwt::create_token, token_wrapper::TokenWrapper,
    },
};

use super::{RequestLoginUser, ResponseDataUser, ResponseUser, UserQueries};

impl UserQueries {
    pub async fn login(
        db: DB,
        jwt_secret: TokenWrapper,
        request_user: RequestLoginUser,
    ) -> Result<Json<ResponseDataUser>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1;",
            request_user.email
        )
        .fetch_one(&db)
        .await;
        match user {
            Ok(user) => {
                if !verify_password(&request_user.password, &user.password)? {
                    AppError::new(StatusCode::UNAUTHORIZED, "incorrect email and/or password");
                }
                let token = create_token(&jwt_secret.0, user.id, user.role)?;
                let updated_user = sqlx::query!(
                    "UPDATE users SET token = $1 WHERE id = $2 RETURNING *;",
                    token,
                    user.id
                )
                .fetch_one(&db)
                .await?;

                let response_user = ResponseUser {
                    id: updated_user.id,
                    email: updated_user.email,
                    role: updated_user.role,
                    token: updated_user.token.unwrap(),
                };

                Ok(Json(ResponseDataUser {
                    data: response_user,
                }))
            }
            Err(error) => {
                tracing::error!("Problem logging in the user: {}", error);
                Err(eyre::Report::new(error))
            }
        }
    }
}
