use crate::{
    models::user::User,
    utilities::{hash::hash_password, jwt::create_token},
};
use axum::Json;
use eyre::Result;

use crate::{app_state::DB, utilities::token_wrapper::TokenWrapper};

use super::{RequestCreateUser, ResponseDataUser, ResponseUser, UserQueries};

impl UserQueries {
    pub async fn create_user(
        db: DB,
        jwt_secret: TokenWrapper,
        request_user: RequestCreateUser,
        role: String,
    ) -> Result<Json<ResponseDataUser>> {
        let new_user = sqlx::query!(
            "INSERT INTO users (email, password, role) VALUES ($1, $2, $3) RETURNING id;",
            request_user.email,
            hash_password(&request_user.password)?,
            role,
        )
        .fetch_one(&db)
        .await?;

        let token = create_token(&jwt_secret.0, new_user.id, role.clone())?;
        let updated_user = sqlx::query_as!(
            User,
            "UPDATE users SET token = $1 WHERE id = $2 RETURNING *;",
            token,
            new_user.id
        )
        .fetch_one(&db)
        .await?;

        let response_user = ResponseUser {
            id: updated_user.id,
            email: updated_user.email,
            role,
            token,
        };

        Ok(Json(ResponseDataUser {
            data: response_user,
        }))
    }
}
