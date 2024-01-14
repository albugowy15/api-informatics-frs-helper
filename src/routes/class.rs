use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{services::class, AppState};

pub fn class_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(class::classes))
        .route("/:id_kelas", get(class::class_by_id))
}
