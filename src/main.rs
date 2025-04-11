mod models;
mod routes;

const URL_ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    // creating router
    let app = routes::router::create_router();

    // listening to app
    let listener = tokio::net::TcpListener::bind(URL_ADDRESS).await.unwrap();
    println!("Server is running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
