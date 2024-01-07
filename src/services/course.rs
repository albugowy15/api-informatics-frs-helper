use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, Json};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct CourseResponse {
    nama: String,
    semester: u16,
    sks: u16,
}

pub async fn course_handler(
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let response = CourseResponse {
        nama: String::from("jaringan komputer"),
        semester: 5,
        sks: 3,
    };

    Ok(Json(json!(response)))
}
