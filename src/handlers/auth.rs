use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use jsonwebtoken::{EncodingKey, Header, encode};

use tower_cookies::Cookies;

use bcrypt::DEFAULT_COST;

use crate::{
    db::DbPool,
    models::{
        auth::{LoginRequest, RegisterRequest, TokenClaims},
        user::User,
    },
    utils::cookies::set_cookie,
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

// register user logic

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

//login logic

pub async fn login(
    cookies: Cookies,
    State(pool): State<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthMessages> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email=$1")
        .bind(payload.email)
        .fetch_one(&*pool)
        .await
        .map_err(|_e| AuthMessages::BadRequest("Invalid email or username".to_string()))?;

    match verify_password(&payload.password, user.password) {
        Ok(true) => {
            let now = chrono::Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
            let claims: TokenClaims = TokenClaims {
                user_id: user.user_id.to_string(),
                exp,
                iat,
            };
            let encoding_key = match get_rsa_encoding_key() {
                Ok(key) => key,
                Err(err) => {
                    eprintln!("RSA Key Load Error: {:?}", err);
                    return Err(AuthMessages::ServerError(
                        "Internal server error".to_string(),
                    ));
                }
            };

            // Attempt to encode the token
            let token = match encode(
                &Header::new(jsonwebtoken::Algorithm::RS256),
                &claims,
                &encoding_key,
            ) {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("JWT Encode Error: {:?}", err);
                    return Err(AuthMessages::ServerError(
                        "Internal Server Error.".to_string(),
                    ));
                }
            };

            set_cookie(
                "auth-token".to_string(),
                token,
                true,
                "/".to_string(),
                &cookies,
            )
            .await;

            Ok(AuthMessages::SuccessMessage("User logged in".to_string()))
        }
        Ok(false) => Err(AuthMessages::BadRequest(
            "Invalid email or username".to_string(),
        )),
        Err(_) => Ok(AuthMessages::ServerError(
            "Something went wrong".to_string(),
        )),
    }
}

// Helper function
pub fn get_rsa_encoding_key() -> Result<EncodingKey, Box<dyn std::error::Error>> {
    let path = std::env::var("PRIVATE_KEY_PATH")?;
    let pem = std::fs::read(path)?;
    Ok(EncodingKey::from_rsa_pem(&pem)?)
}

fn verify_password(
    password: &String,
    hashed_password: String,
) -> Result<bool, bcrypt::BcryptError> {
    let result = bcrypt::verify(password, &hashed_password)?;
    Ok(result)
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
