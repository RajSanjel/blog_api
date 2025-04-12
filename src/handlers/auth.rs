use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bcrypt::DEFAULT_COST;

use crate::{
    db::DbPool,
    models::{auth::RegisterRequest, user::User},
};

pub enum AuthMessages {
    BadRequest(String),
    SuccessMessage(String),
    ServerError(String),
}

impl IntoResponse for AuthMessages {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::SuccessMessage(message) => (StatusCode::OK, message).into_response(),
            Self::ServerError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed = bcrypt::hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

async fn check_if_user_exists(
    pool: &DbPool,
    username: &str,
    email: &str,
) -> Result<bool, sqlx::Error> {
    let user_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)",
        username,
        email
    )
    .fetch_one(&**pool)
    .await?;

    Ok(user_exists.unwrap_or(false))
}

pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AuthMessages> {
    let email = payload.email;
    let password = payload.password;
    let username = payload.username;
    let hashed_password = hash_password(&password)
        .map_err(|_| AuthMessages::ServerError("Hashing error".to_string()))?;
    if !email.contains("@") {
        return Err(AuthMessages::BadRequest("Invalid Email".to_string()));
    }

    if username.is_empty() || username.len() < 3 {
        return Err(AuthMessages::BadRequest("Invalid Username".to_string()));
    }

    if password.len() < 8 {
        return Err(AuthMessages::BadRequest("Invalid Password".to_string()));
    }

    match check_if_user_exists(&pool, &username, &email).await {
        Ok(true) => Err(AuthMessages::BadRequest(
            "Invalid email or username".to_string(),
        )),
        Ok(false) => {
            sqlx::query_as::<_, User>(
                "INSERT INTO users (email, username, password) VALUES ($1, $2, $3) RETURNING *",
            )
            .bind(&email)
            .bind(&username)
            .bind(&hashed_password)
            .fetch_one(&*pool)
            .await
            .map_err(|_e| AuthMessages::ServerError("Something went wrong".to_string()))?;

            Ok(AuthMessages::SuccessMessage(
                "User registered successfully!".to_string(),
            ))
        }
        Err(_) => Err(AuthMessages::ServerError(
            "Something went wrong.".to_string(),
        )),
    }
}
