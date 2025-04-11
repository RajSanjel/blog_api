use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::sync::Arc;
pub type DbPool = Arc<PgPool>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("URL must be provided in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&database_url)
        .await?;

    Ok(Arc::new(pool))
}
