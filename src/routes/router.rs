use super::check_health::check_health;
use crate::db::DbPool;
use crate::handlers::auth::register;
use axum::{Router, routing::get, routing::post};

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/check_health", get(check_health))
        .route("/auth/register", post(register))
        .with_state(pool)
}
