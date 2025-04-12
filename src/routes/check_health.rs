use axum::response::{IntoResponse, Json};

pub async fn check_health() -> impl IntoResponse {
    Json("Health check passed").into_response()
}
