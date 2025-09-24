use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, services::course};

pub fn course_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(course::courses))
        .route("/dosen", get(course::courses_with_lecturers))
        .route("/kelas", get(course::courses_with_classes))
        .route("/{id_matkul}", get(course::course_by_id))
        .route(
            "/{id_matkul}/dosen",
            get(course::course_by_id_with_lecturers),
        )
        .route("/{id_matkul}/kelas", get(course::course_by_id_with_classes))
}
