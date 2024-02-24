use askama_axum::IntoResponse;
use axum::{http::HeaderMap, response::Html};
use reqwest::header::SET_COOKIE;

use super::jwt::Jwt;

pub async fn logout() -> impl IntoResponse {
    let jwt = Jwt::destroy();

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, jwt.to_string().parse().unwrap());

    // I cannot redirect bc. the cookie won't set, and sending the body of
    // the page does weird shit with HTMX, so while I don't understand how
    // it works, that'll be the way 🫡
    (
        headers,
        Html("<script>window.location.href = '/login'</script>"),
    )
}
