use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use super::{auth::user::User, STATIC_DIR};

fn get_file(file: &str, new_path: Option<&str>) -> impl IntoResponse {
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
            .header("HX-Replace-Url", new_path.unwrap_or(path))
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

pub async fn login_page(user: Option<User>) -> impl IntoResponse {
    if user.is_some() {
        return get_file("home.html", Some("/"));
    }
    get_file("login.html", Some("/login"))
}

pub async fn index(user: Option<User>) -> impl IntoResponse {
    if user.is_none() {
        return Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/login")
            .body(body::boxed(Empty::new()))
            .unwrap();
    }

    let index = STATIC_DIR
        .get_file("home.html")
        .expect("home.html should exist");

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .body(body::boxed(Full::from(index.contents())))
        .unwrap()
}

pub async fn static_path(_user: User, Path(path): Path<String>) -> impl IntoResponse {
    get_file(&path, Some(&path.to_owned().trim_end_matches(".html")))
}
