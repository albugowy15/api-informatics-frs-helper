use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;
pub mod repository;
pub mod route;
pub mod services;

#[tokio::main]
async fn main() {
    // load env
    dotenv::dotenv().expect("Error load .env");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api-informatics-frs-helper=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // setup app route
    let app = route::get_routes().await;

    // attach http listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listeing on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
