use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, services::class};

pub fn class_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(class::classes))
        .route("/{id_kelas}", get(class::class_by_id))
}
