use std::sync::Arc;

use api_informatics_frs_helper::{db::DbConnection, route, AppState};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "api_informatics_frs_helper=debug,tower_http=debug,sqlx=error".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    if let Err(err) = dotenvy::dotenv() {
        tracing::debug!("Error load env: {}", err)
    };
    let db = DbConnection::new().await.unwrap();
    let shared_state = Arc::new(AppState { db_pool: db.pool });
    let app = route::get_routes(Arc::clone(&shared_state)).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(Arc::clone(&shared_state)))
        .await
        .unwrap();
}

async fn shutdown_signal(shared_state: Arc<AppState>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => shared_state.db_pool.close().await,
        _ = terminate => shared_state.db_pool.close().await,
    }
}
