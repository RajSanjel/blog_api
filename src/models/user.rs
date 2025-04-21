use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct UserData {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
}
