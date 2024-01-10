use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::json;

use crate::{
    repository::{
        class_repository::{ClassRepository, ClassWithSubjectName},
        course_repository::{Course, CourseRepository},
        lecturer_repository::{LecturerRepository, LecturerWithClasses, LecturerWithCourses},
    },
    route::AppState,
};

use super::{display_err, ErrorViews, JsonResponse, RouteHandler, SuccessResponse};

pub async fn lecturers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let mut lecturers = match lecturer_repo.get_lecturers().await {
        Ok(lecturers) => lecturers,
        _ => {
            return Err(display_err(ErrorViews::Internal));
        }
    };
    if let Some(fullname_param) = params.get("nama") {
        lecturers.retain(|lecturer| {
            lecturer
                .nama
                .to_lowercase()
                .contains(&fullname_param.to_lowercase())
        });
    }
    if let Some(code_param) = params.get("kode") {
        lecturers.retain(|lecturer| lecturer.kode.to_lowercase() == *code_param.to_lowercase());
    }
    Ok(Json(json!(SuccessResponse {
        total_results: lecturers.len(),
        data: lecturers,
    })))
}

pub async fn lecturer_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_dosen): Path<String>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturer = match lecturer_repo.get_lecturer_by_id(&id_dosen).await {
        Ok(lecturer) => lecturer,
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return Err(display_err(ErrorViews::NotFound("Dosen")));
            }
            _ => return Err(display_err(ErrorViews::Internal)),
        },
    };
    Ok(Json(json!(lecturer)))
}

pub async fn lecturer_by_id_with_classes(
    State(state): State<Arc<AppState>>,
    Path(id_dosen): Path<String>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturer = match lecturer_repo.get_lecturer_by_id(&id_dosen).await {
        Ok(data) => data,
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(display_err(ErrorViews::NotFound("Dosen"))),
            _ => return Err(display_err(ErrorViews::Internal)),
        },
    };
    let class_repo = ClassRepository::new(&state.db_pool);
    let classes = class_repo
        .get_classes_by_lecturer_id(&lecturer.id)
        .await
        .unwrap_or_default();
    let response = LecturerWithClasses::<Vec<ClassWithSubjectName>> {
        id: lecturer.id,
        nama: lecturer.nama,
        kode: lecturer.kode,
        kelas: classes,
    };
    Ok(Json(json!(response)))
}

pub async fn lecturer_by_id_with_courses(
    State(state): State<Arc<AppState>>,
    Path(id_dosen): Path<String>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturer = match lecturer_repo.get_lecturer_by_id(&id_dosen).await {
        Ok(data) => data,
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(display_err(ErrorViews::NotFound("Dosen"))),
            _ => return Err(display_err(ErrorViews::Internal)),
        },
    };
    let course_repo = CourseRepository::new(&state.db_pool);
    let courses = course_repo
        .get_courses_by_lecturer_id(&lecturer.id)
        .await
        .unwrap_or_default();
    let response = LecturerWithCourses::<Vec<Course>> {
        id: lecturer.id,
        kode: lecturer.kode,
        nama: lecturer.nama,
        matkul: courses,
    };
    Ok(Json(json!(response)))
}
