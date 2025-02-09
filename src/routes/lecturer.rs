use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{services::lecturer, AppState};

pub fn lecturer_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(lecturer::lecturers))
        .route("/matkul", get(lecturer::lecturers_with_courses))
        .route("/kelas", get(lecturer::lecturers_with_classes))
        .route("/{id_dosen}", get(lecturer::lecturer_by_id))
        .route(
            "/{id_dosen}/kelas",
            get(lecturer::lecturer_by_id_with_classes),
        )
        .route(
            "/{id_dosen}/matkul",
            get(lecturer::lecturer_by_id_with_courses),
        )
}
