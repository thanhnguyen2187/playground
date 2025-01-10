use axum::http::StatusCode;
use axum::response::{IntoResponse, Html};
use hypertext::{rsx, Renderable, html_elements, GlobalAttributes};
use crate::templates;

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn default_fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub async fn index() -> Html<String> {
    let body = rsx! {
        <h1>"Hello, world!"</h1>
    };
    let template = templates::index("Hello, world!".to_string(), body).render();

    Html(template.into_inner())
}