use axum::{http::StatusCode, Json};
use serde_json::Value;
pub mod class_service;
pub mod course_service;
pub mod home_service;
pub mod lecturer_service;
pub mod swagger_service;

pub type JsonResponse = Json<Value>;
pub type RouteHandler<T> = Result<T, (StatusCode, String)>;
