use std::sync::Arc;

use axum::{error_handling::HandleErrorLayer, routing::get, BoxError, Router};
use hyper::StatusCode;
use tower::{buffer::BufferLayer, ServiceBuilder};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{middleware, routes, services, AppState};

pub async fn get_routes(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(services::home::home))
        .route("/swagger", get(services::swagger::swagger))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest("/v1/matkul", routes::course::course_routes())
        .nest("/v1/dosen", routes::lecturer::lecturer_routes())
        .nest("/v1/kelas", routes::class::class_routes())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Internal server error: {}", err),
                    )
                }))
                .layer(TraceLayer::new_for_http())
                .layer(middleware::cors())
                .layer(middleware::compression())
                .layer(middleware::request_timeout())
                .layer(BufferLayer::new(1024))
                .layer(middleware::rate_limit()),
        )
        .with_state(shared_state)
}
