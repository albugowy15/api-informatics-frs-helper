use std::{collections::HashMap, sync::Arc};

use axum::extract::{Path, Query, State};

use crate::{
    model::{
        class_model::ClassWithSubjectName,
        course_model::Course,
        lecturer_model::{LecturerWithClasses, LecturerWithCourses},
    },
    repository::{
        class_repository::ClassRepository, course_repository::CourseRepository,
        lecturer_repository::LecturerRepository,
    },
    route::AppState,
};

use super::{DataResponse, ErrorViews, IntoJson, JsonResponse, RouteHandler};

pub async fn lecturers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturers = lecturer_repo
        .get_lecturers_with_filter(&params)
        .await
        .map_err(|_| ErrorViews::Internal)?;
    Ok(DataResponse::new(lecturers.len(), lecturers).into_json())
}

pub async fn lecturers_with_courses(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturers_courses = lecturer_repo
        .get_lecturers_with_courses()
        .await
        .map_err(|_| ErrorViews::Internal)?;
    Ok(DataResponse::new(lecturers_courses.len(), lecturers_courses).into_json())
}

pub async fn lecturers_with_classes(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturers_classes = lecturer_repo
        .get_lecturers_with_classes()
        .await
        .map_err(|_| ErrorViews::Internal)?;
    Ok(DataResponse::new(lecturers_classes.len(), lecturers_classes).into_json())
}

pub async fn lecturer_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_dosen): Path<String>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturer = lecturer_repo
        .get_lecturer_by_id(&id_dosen)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ErrorViews::NotFound("Dosen"),
            _ => ErrorViews::Internal,
        })?;
    Ok(lecturer.into_json())
}

pub async fn lecturer_by_id_with_classes(
    State(state): State<Arc<AppState>>,
    Path(id_dosen): Path<String>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturer = lecturer_repo
        .get_lecturer_by_id(&id_dosen)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ErrorViews::NotFound("Dosen"),
            _ => ErrorViews::Internal,
        })?;
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
    Ok(response.into_json())
}

pub async fn lecturer_by_id_with_courses(
    State(state): State<Arc<AppState>>,
    Path(id_dosen): Path<String>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturer = lecturer_repo
        .get_lecturer_by_id(&id_dosen)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ErrorViews::NotFound("Dosen"),
            _ => ErrorViews::Internal,
        })?;
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
    Ok(response.into_json())
}
