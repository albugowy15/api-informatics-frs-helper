use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::json;

use super::ErrorViews;
use crate::{
    repository::{
        class_repository::{ClassRepository, CompactClass},
        course_repository::{CourseRepository, CourseWithClass, CourseWithLecturer},
        lecturer_repository::{Lecturer, LecturerRepository},
    },
    route::AppState,
};

use super::{display_err, JsonResponse, RouteHandler, SuccessResponse};

pub async fn courses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let mut courses = match course_repo.get_courses().await {
        Ok(courses) => courses,
        _ => {
            return Err(display_err(ErrorViews::Internal));
        }
    };
    if let Some(course_name_param) = params.get("nama") {
        courses.retain(|course| {
            course
                .nama
                .to_lowercase()
                .contains(&course_name_param.to_lowercase())
        })
    }
    if let Some(semester_param) = params.get("semester") {
        let parse_semester_param = match semester_param.parse::<i8>() {
            Ok(num) => num,
            _ => {
                return Err(display_err(ErrorViews::BadRequest(String::from(
                    "semester wajib berupa angka",
                ))))
            }
        };
        courses.retain(|course| course.semester == parse_semester_param)
    }
    if let Some(sks_param) = params.get("sks") {
        let parse_sks_param = match sks_param.parse::<i8>() {
            Ok(num) => num,
            _ => {
                return Err(display_err(ErrorViews::BadRequest(String::from(
                    "sks wajib berupa angka",
                ))))
            }
        };
        courses.retain(|course| course.sks == parse_sks_param)
    }
    let response = SuccessResponse {
        total_results: courses.len(),
        data: courses,
    };
    Ok(Json(json!(response)))
}

pub async fn course_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = match course_repo.get_course_by_id(&id_matkul).await {
        Ok(course) => course,
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(display_err(ErrorViews::NotFound("Matkul"))),
            _ => return Err(display_err(ErrorViews::Internal)),
        },
    };
    Ok(Json(json!(course)))
}

pub async fn courses_with_lecturers(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let courses_lecturers = match course_repo.get_courses_with_lecturers().await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    let response = SuccessResponse {
        total_results: courses_lecturers.len(),
        data: courses_lecturers,
    };
    Ok(Json(json!(response)))
}

pub async fn courses_with_classes(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let courses_classes = match course_repo.get_courses_with_classes().await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    let response = SuccessResponse {
        total_results: courses_classes.len(),
        data: courses_classes,
    };
    Ok(Json(json!(response)))
}

pub async fn course_with_classes_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = match course_repo.get_course_by_id(&id_matkul).await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    let class_repo = ClassRepository::new(&state.db_pool);
    let classes = match class_repo.get_classes_by_course_id(&course.id).await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    let response: CourseWithClass<Vec<CompactClass>> = CourseWithClass {
        id: course.id,
        nama: course.nama,
        semester: course.semester,
        sks: course.sks,
        kelas: classes,
    };
    Ok(Json(json!(response)))
}

pub async fn course_with_lecturers_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = match course_repo.get_course_by_id(&id_matkul).await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturers = match lecturer_repo.get_lecturers_by_course_id(&course.id).await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    let response: CourseWithLecturer<Vec<Lecturer>> = CourseWithLecturer {
        id: course.id,
        nama: course.nama,
        semester: course.semester,
        sks: course.sks,
        dosen: lecturers,
    };
    Ok(Json(json!(response)))
}
