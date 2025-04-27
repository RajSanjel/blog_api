use crate::{
    db::DbPool, response::server_response::ServerResponse, utils::db_functions::get_user_data,
};
use axum::{Extension, extract::State, response::IntoResponse};
use serde::Serialize;

// Response structure for the profile endpoint
#[derive(Serialize)]
pub struct ProfileResponse {
    status: String,
    message: String,
    data: UserProfile,
}

#[derive(Serialize)]
pub struct UserProfile {
    username: String,
    email: String,
}

pub async fn get_user(
    State(pool): State<DbPool>,
    Extension(user_id): Extension<String>,
) -> Result<impl IntoResponse, ServerResponse<()>> {
    match get_user_data(user_id, pool).await {
        Ok(user) => Ok(ServerResponse::SuccessMessage(
            "Success".to_string(),
            Some(user),
        )),
        Err(_) => Err(ServerResponse::ServerError(
            "Something went wrong".to_string(),
        )),
    }
}
