use crate::{
    db::DbPool,
    models::user::{User, UserData},
    response::server_response::ServerResponse,
    utils::{cookies::read_cookies, jwt::jwt_decode},
};
use axum::{extract::State, response::IntoResponse};
use tower_cookies::Cookies;
use uuid::Uuid;

// GET /user
pub async fn get_user(
    State(pool): State<DbPool>,
    cookies: Cookies,
) -> Result<impl IntoResponse, ServerResponse<()>> {
    match read_cookies(cookies, "auth-token").await {
        Ok(token) => match jwt_decode(token) {
            Ok(decoded) => {
                let user = get_user_data(decoded.user_id, pool).await?;
                Ok(ServerResponse::SuccessMessage(
                    "Success".to_string(),
                    Some(user),
                ))
            }
            Err(_) => Err(ServerResponse::BadRequest(
                "Invalid token".to_string(),
                None,
            )),
        },
        Err(_) => Err(ServerResponse::BadRequest(
            "No token provided".to_string(),
            None,
        )),
    }
}

// DB Query for a user by ID
pub async fn get_user_data(user_id: String, pool: DbPool) -> Result<UserData, ServerResponse<()>> {
    let user_id_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ServerResponse::BadRequest("Something went wrong".to_string(), None))?;
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1")
        .bind(user_id_uuid)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ServerResponse::BadRequest("User not found".to_string(), None)
            }
            _ => ServerResponse::ServerError(e.to_string()),
        })?;
    let user_data = UserData {
        user_id: user.user_id,
        email: user.email,
        username: user.username,
    };
    Ok(user_data)
}
