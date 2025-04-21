mod db;
mod handlers;
mod models;
mod response;
mod routes;
mod utils;
const URL_ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Connect to the database
    let pool = db::init_db().await?;

    let app = routes::router::create_router(pool);

    // listening to app
    let listener = tokio::net::TcpListener::bind(URL_ADDRESS).await?;
    println!("Server is running on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
