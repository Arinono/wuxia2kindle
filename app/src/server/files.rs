use axum::{http::{HeaderMap, StatusCode, header::CONTENT_TYPE, HeaderValue}, extract::{State, Path}, response::{IntoResponse, Response}, body::{self, Full, Empty}};
use sqlx::PgPool;

use super::{
    STATIC_DIR,
    auth::cookie::get_cookie,
};

pub async fn index(headers: HeaderMap, State(pool): State<PgPool>) -> impl IntoResponse {
    let user = get_cookie(&headers, &pool).await;
    println!("User: {:?}", user);
    if user.is_none() {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .body(body::boxed(Full::from(
                STATIC_DIR.get_file("login.html").unwrap().contents(),
            )))
            .unwrap();
    }

    let index = STATIC_DIR
        .get_file("index.html")
        .expect("index.html should exist");

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .body(body::boxed(Full::from(index.contents())))
        .unwrap()
}

fn get_file(file: &str) -> impl IntoResponse {
    let path = file.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    let file = STATIC_DIR.get_file(path);
    match file {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    get_file(&path)
}
