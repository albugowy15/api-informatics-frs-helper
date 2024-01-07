use std::collections::HashMap;

use axum::{extract::Query, Json};
use serde_json::json;

use super::RouteHandler;

pub async fn lecturer_handler(Query(_params): Query<HashMap<String, String>>) -> RouteHandler {
    Ok(Json(json!({})))
}
