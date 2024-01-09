use axum::{http::StatusCode, Json};
use serde::Serialize;
use serde_json::Value;
pub mod class_service;
pub mod course_service;
pub mod home_service;
pub mod lecturer_service;
pub mod swagger_service;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    total_results: usize,
    data: T,
}

pub type JsonResponse = Json<Value>;
pub type RouteHandler<T> = Result<T, (StatusCode, String)>;

pub enum ErrorViews<'a> {
    Internal,
    NotFound(&'a str),
    BadRequest(String),
}

fn display_err(variant: ErrorViews) -> (StatusCode, String) {
    match variant {
        ErrorViews::Internal => (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal server error"),
        ),
        ErrorViews::NotFound(field) => {
            (StatusCode::NOT_FOUND, format!("{} tidak ditemukan", field))
        }
        ErrorViews::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
    }
}
