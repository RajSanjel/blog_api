use axum::{Router, routing::get};

use super::check_health::check_health;

pub fn create_router() -> Router {
    Router::new().route("/check_health", get(check_health()))
}
