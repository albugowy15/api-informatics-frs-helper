use std::{sync::Arc, time::Duration};

use axum::{error_handling::HandleErrorLayer, routing::get, BoxError, Router};
use hyper::StatusCode;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{
    db::{DbConnection, DbPool},
    services,
};

pub struct AppState {
    pub db_pool: DbPool,
}

pub async fn get_routes() -> Router {
    let db = DbConnection::create_db_connection().await.unwrap();
    let shared_state = Arc::new(AppState { db_pool: db });

    Router::new()
        .route("/", get(services::home_service::home_handler))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/swagger", get(services::swagger_service::swagger_handler))
        .route("/v1/matkul", get(services::course_service::course_handler))
        .route(
            "/v1/matkul/:id_matkul",
            get(services::course_service::course_with_id_handler),
        )
        .route(
            "/v1/dosen",
            get(services::lecturer_service::lecturer_handler),
        )
        .route(
            "/v1/dosen/:id_dosen",
            get(services::lecturer_service::lecturer_with_id_handler),
        )
        .route("/v1/kelas", get(services::class_service::class_handler))
        .route(
            "/v1/kelas/:id_kelas",
            get(services::class_service::class_by_id_handler),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Internal server error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
        )
        .with_state(shared_state)
}
