use askama::Template;
use axum::response::IntoResponse;

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}
