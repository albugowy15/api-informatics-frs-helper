pub mod db;
pub mod repository;
pub mod route;
pub mod services;

#[tokio::main]
async fn main() {
    // load env
    dotenv::dotenv().expect("Error load .env");

    // setup app route
    let app = route::get_routes().await;

    // attach http listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // serve
    axum::serve(listener, app).await.unwrap();
}
