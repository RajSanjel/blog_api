use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Post {
    pub post_id: Uuid,
    pub title: String,
    pub body: String,
    pub slug: String,
    pub author_id: i32,
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
