use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    Json,
};
use db::DbPool;
use hyper::StatusCode;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::mysql::MySqlRow;

pub mod db;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod route;
pub mod routes;
pub mod services;

/// `AppState` is a struct that represents the application state.
///
/// It includes a database pool, which can be used to execute database queries.
///
/// # Fields
///
/// * `db_pool`: The database pool used by the application. A database pool is a cache of
/// database connections maintained so that the connections can be reused when needed.
pub struct AppState {
    pub db_pool: DbPool,
}

/// `JsonResponse` is a type alias for a JSON response. The `Json` wrapper is used to
/// automatically set the Content-Type response header to `application/json`.
pub type JsonResponse = Json<Value>;

/// `RouteHandler` is a type alias for the result of a route handler. It either returns
/// a successful result of type `T`, or an error represented as a tuple of a `StatusCode`
/// and a `String` message.
pub type RouteHandler<T> = Result<T, (StatusCode, String)>;

/// `FromRows` is a trait for types that can be created from a slice of `MySqlRow`.
///
/// This trait provides a single method `from_rows`, which takes a slice of `MySqlRow`
/// and returns a `Result` with the type implementing `FromRows` or an `sqlx::Error`.
///
/// The `Self: Sized` bound is necessary because the method returns a value of type `Self`.
pub trait FromRows {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error>
    where
        Self: Sized;
}

/// `DataResponse` is a generic struct that represents a data response.
///
/// It includes the total number of results and the data itself. The data can be of any type
/// that implements the `Serialize` trait.
#[derive(Serialize)]
pub struct DataResponse<T> {
    total_results: usize,
    data: T,
}

/// This implementation allows a `DataResponse` to be converted into a `Json<Value>`.
/// This is useful for sending `DataResponse` as a JSON response in a web server.
impl<T: Serialize> From<DataResponse<T>> for Json<Value> {
    fn from(value: DataResponse<T>) -> Self {
        Json(json!(value))
    }
}

/// This implementation provides a method to create a new `DataResponse`.
impl<T: Serialize> DataResponse<T> {
    /// Creates a new `DataResponse` with the given total results and data.
    pub fn new(total_results: usize, data: T) -> Self {
        Self {
            total_results,
            data,
        }
    }
}

/// `ErrorViews` is an enum representing different types of errors that can occur in the application.
/// Each variant corresponds to a different type of error:
/// - `Internal` represents an internal server error.
/// - `NotFound` represents a not found error, with a string slice that specifies what was not found.
/// - `BadRequest` represents a bad request error, with a string that provides more details about the error.
pub enum ErrorViews<'a> {
    Internal,
    NotFound(&'a str),
    BadRequest(String),
}

/// This implementation allows an `ErrorViews` to be converted into a tuple of `StatusCode` and `String`.
/// This is useful for sending `ErrorViews` as a HTTP response in a web server.
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

/// `HtmlTemplate` is a struct that wraps a type `T` which implements the `Template` trait.
/// This struct is used to render the template into HTML and convert it into a response.
pub struct HtmlTemplate<T>(pub T);

/// This implementation allows an `HtmlTemplate` to be converted into a `axum::response::Response`.
/// This is useful for sending `HtmlTemplate` as a HTTP response in a web server.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    /// Converts the `HtmlTemplate` into a `axum::response::Response`.
    ///
    /// If the template is successfully rendered into HTML, it returns a `Html` response.
    /// If the template fails to render, it returns an `INTERNAL_SERVER_ERROR` response with a message detailing the error.
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
