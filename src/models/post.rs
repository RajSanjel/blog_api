use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Post {
    pub post_id: Uuid,
    pub title: String,
    pub body: String,
    pub slug: String,
    pub author_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct PostResonse {
    pub title: String,
    pub body: String,
    pub slug: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub body: String,
}
