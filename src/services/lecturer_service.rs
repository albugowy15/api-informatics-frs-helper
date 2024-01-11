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

use super::{display_err, DataResponse, ErrorViews, IntoJson, JsonResponse, RouteHandler};

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
    Ok(DataResponse::new(lecturers.len(), lecturers).into_json())
}

pub async fn lecturers_with_courses(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturers_courses = match lecturer_repo.get_lecturers_with_courses().await {
        Ok(data) => data,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    Ok(DataResponse::new(lecturers_courses.len(), lecturers_courses).into_json())
}

pub async fn lecturers_with_classes(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturers_classes = match lecturer_repo.get_lecturers_with_classes().await {
        Ok(data) => data,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    Ok(DataResponse::new(lecturers_classes.len(), lecturers_classes).into_json())
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
    Ok(lecturer.into_json())
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
    Ok(response.into_json())
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
    Ok(response.into_json())
}
