use askama::Template;
use axum::response::IntoResponse;

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "swagger.html")]
struct SwaggerTemplate;

pub async fn swagger() -> impl IntoResponse {
    let template = SwaggerTemplate {};
    HtmlTemplate(template)
}
