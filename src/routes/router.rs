use axum::{Router, routing::get};

use crate::db::DbPool;

use super::check_health::check_health;

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/check_health", get(check_health()))
        .with_state(pool)
}
