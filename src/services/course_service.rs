use std::collections::HashMap;

use axum::{extract::Query, Json};
use serde::Serialize;
use serde_json::json;

use super::RouteHandler;

#[derive(Serialize)]
struct CourseResponse {
    nama: String,
    semester: u16,
    sks: u16,
}

pub async fn course_handler(Query(_params): Query<HashMap<String, String>>) -> RouteHandler {
    let response = CourseResponse {
        nama: String::from("jaringan komputer"),
        semester: 5,
        sks: 3,
    };

    Ok(Json(json!(response)))
}
