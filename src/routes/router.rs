use super::check_health::check_health;
use crate::db::DbPool;
use crate::handlers::auth::{login, logout_user, register};
use crate::handlers::post::{blog_post, get_blog};
use crate::handlers::user::get_user;
use crate::middleware::verify_user::verify_user;

use axum::{Router, routing::get, routing::post};
use axum::{http, middleware};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub fn create_router(pool: DbPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:5500".parse().unwrap()]) // assuming it's our frontend
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
        .allow_credentials(true);

    let auth_router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout_user));

    let api_router = Router::new()
        .route("/check_health", get(check_health))
        .route("/post/{id}", get(get_blog));

    let protected_api_router = Router::new()
        .route("/get_user", get(get_user))
        .route("/post", post(blog_post))
        .layer(middleware::from_fn(verify_user));

    Router::new()
        .nest("/auth", auth_router)
        .nest("/api", api_router)
        .nest("/api", protected_api_router)
        .with_state(pool)
        .layer(CookieManagerLayer::new())
        .layer(cors)
}
