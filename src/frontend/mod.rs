use axum::{
    body::{boxed, Full},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/frontend/public"]
struct EmbeddedFrontendFS;

pub async fn frontend_handler(request: axum::http::Request<axum::body::Body>) -> impl IntoResponse {
    let path = request.uri().path().trim_start_matches('/');
    let file = EmbeddedFrontendFS::get(path);
    match file {
        Some(content) => {
            let payload = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(payload)
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(boxed(Full::from("404 not found")))
            .unwrap(),
    }
}
