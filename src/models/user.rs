use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Domain Entity for User in the database
#[derive(Serialize, Deserialize, Clone, Default, FromRow)]
pub struct User {
    pub id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub image: Option<String>,
    pub role: String,
    pub description: Option<String>,
    pub token: Option<String>,
}
