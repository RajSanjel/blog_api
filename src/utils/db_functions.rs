use uuid::Uuid;

use crate::{
    db::DbPool,
    models::user::{User, UserData},
};

pub async fn get_user_data(user_id: String, pool: DbPool) -> Result<UserData, String> {
    let user_id_uuid = Uuid::parse_str(&user_id).map_err(|_| "Invalid UUID")?;
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1")
        .bind(user_id_uuid)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => "User not found.".to_string(),
            _ => e.to_string(),
        })?;
    let user_data = UserData {
        user_id: user.user_id,
        email: user.email,
        username: user.username,
    };
    Ok(user_data)
}
