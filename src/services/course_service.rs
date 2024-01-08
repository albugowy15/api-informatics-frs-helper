use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{repository, route::AppState};

use super::{JsonResponse, RouteHandler, SuccessResponse};

pub async fn course_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = repository::course_repository::CourseRepository::new(&state.db_pool);
    let mut courses = match course_repo.get_courses().await {
        Ok(courses) => courses,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ));
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
                return Err((
                    StatusCode::BAD_REQUEST,
                    String::from("semester wajib berupa angka"),
                ))
            }
        };
        courses.retain(|course| course.semester == parse_semester_param)
    }
    if let Some(sks_param) = params.get("sks") {
        let parse_sks_param = match sks_param.parse::<i8>() {
            Ok(num) => num,
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    String::from("sks wajib berupa angka"),
                ))
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

pub async fn course_with_id_handler(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = repository::course_repository::CourseRepository::new(&state.db_pool);
    let course = match course_repo.get_course_by_id(&id_matkul).await {
        Ok(course) => course,
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return Err((
                    StatusCode::NOT_FOUND,
                    String::from("Matkul tidak ditemukan"),
                ))
            }
            _ => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Internal server error"),
                ))
            }
        },
    };

    Ok(Json(json!(course)))
}
