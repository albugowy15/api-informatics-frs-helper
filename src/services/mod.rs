use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub mod class_service;
pub mod course_service;
pub mod home_service;
pub mod lecturer_service;

#[derive(Deserialize, Serialize)]
pub struct SuccessResponse<T> {
    total_results: usize,
    data: T,
}

pub type RouteHandler = Result<Json<Value>, (StatusCode, String)>;
