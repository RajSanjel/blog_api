use super::check_health::check_health;
use crate::db::DbPool;
use crate::handlers::auth::{login, register};
use crate::handlers::user::get_user;

use axum::{Router, routing::get, routing::post};
use tower_cookies::CookieManagerLayer;

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/check_health", get(check_health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/api/get_user", get(get_user))
        .layer(CookieManagerLayer::new())
        .with_state(pool)
}
