use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

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
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state)
}
