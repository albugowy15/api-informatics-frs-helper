use std::env;

use askama::Template;
use axum::response::{Html, IntoResponse};
use hyper::StatusCode;

#[derive(Template)]
#[template(path = "swagger.html")]
struct SwaggerTemplate {
    open_api_json_url: String,
}

struct HtmlTemplate<T>(T);

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

pub async fn swagger_handler() -> impl IntoResponse {
    let api_url = env::var("API_URL").unwrap();
    let template = SwaggerTemplate {
        open_api_json_url: format!("{api_url}/assets/openapi.json"),
    };
    HtmlTemplate(template)
}
