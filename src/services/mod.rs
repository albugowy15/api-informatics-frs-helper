use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::Serialize;
use serde_json::{json, Value};
pub mod class_service;
pub mod course_service;
pub mod home_service;
pub mod lecturer_service;
pub mod swagger_service;

pub trait IntoJson
where
    Self: Serialize,
{
    fn into_json(self) -> JsonResponse
    where
        Self: Sized,
    {
        Json(json!(self))
    }
}
#[derive(Serialize)]
pub struct DataResponse<T> {
    total_results: usize,
    data: T,
}
impl<T: Serialize> IntoJson for DataResponse<T> {}
impl<T: Serialize> DataResponse<T> {
    fn new(total_results: usize, data: T) -> Self {
        Self {
            total_results,
            data,
        }
    }
}

pub type JsonResponse = Json<Value>;
pub type RouteHandler<T> = Result<T, (StatusCode, String)>;

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

pub struct HtmlTemplate<T>(T);

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
