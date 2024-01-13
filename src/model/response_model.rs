use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct DataResponse<T> {
    total_results: usize,
    data: T,
}
impl<T: Serialize> From<DataResponse<T>> for Json<Value> {
    fn from(value: DataResponse<T>) -> Self {
        Json(json!(value))
    }
}
impl<T: Serialize> DataResponse<T> {
    pub fn new(total_results: usize, data: T) -> Self {
        Self {
            total_results,
            data,
        }
    }
}

pub enum ErrorViews<'a> {
    Internal,
    NotFound(&'a str),
    BadRequest(String),
}

impl<'a> From<ErrorViews<'a>> for (StatusCode, String) {
    fn from(val: ErrorViews<'a>) -> Self {
        match val {
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
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
