use std::collections::HashMap;

use axum::extract::Query;

pub async fn lecturer_handler(Query(_params): Query<HashMap<String, String>>) -> &'static str {
    "Lecturer Handler"
}
