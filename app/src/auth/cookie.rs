use std::collections::HashMap;

use axum::http::HeaderMap;
use sqlx::PgPool;

use crate::auth::jwt::JWT;

use super::user::User;

pub static COOKIE_NAME: &str = "wuxia2kindle_session";

pub async fn get_cookie(headers: &HeaderMap, pool: &PgPool) -> Option<User> {
    println!("Headers: {:?}", headers);
    let cookie_header = headers.get("Cookie");

    if cookie_header.is_none() {
        return None;
    }

    let cookie_header_string = cookie_header
        .unwrap()
        .to_str()
        .expect("Cookie should be a string");

    let cookies = cookie_header_string
        .to_string()
        .split(';')
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>()
        .iter()
        .map(|s| {
            let mut split = s.split('=');
            let key = split.next().unwrap().to_owned();
            let value = split.next().unwrap().to_owned();

            (key, value)
        })
        .collect::<HashMap<String, String>>();

    let session_cookie = cookies.get(COOKIE_NAME);

    if session_cookie.is_none() {
        return None;
    }

    let claims = JWT::verify(session_cookie.unwrap().to_owned());

    if claims.is_err() {
        return None;
    }

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1 LIMIT 1",
        claims.unwrap().sub.unwrap().parse::<i32>().unwrap(),
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    user
}
