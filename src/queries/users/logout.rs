use axum::Json;
use eyre::Result;

use crate::{app_state::DB, models::user::User};

use super::{ResponseLogoutUser, UserQueries};

impl UserQueries {
    pub async fn logout(db: DB, user: User) -> Result<Json<ResponseLogoutUser>> {
        sqlx::query!(
            "UPDATE users SET token = $1 WHERE id = $2;",
            None::<String>,
            user.id
        )
        .execute(&db)
        .await?;

        let logout_response = ResponseLogoutUser {
            email: user.email,
            message: "Successfully logged out!".to_string(),
        };
        Ok(Json(logout_response))
    }
}
