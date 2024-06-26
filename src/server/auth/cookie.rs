use std::collections::HashMap;

use axum::http::HeaderMap;
use models::user::User;
use sqlx::PgPool;

use super::jwt::Jwt;

pub static COOKIE_NAME: &str = "wuxia2kindle_session";

pub async fn get_cookie(headers: &HeaderMap, pool: &PgPool) -> Option<User> {
    let cookie_header = headers.get("Cookie")?;

    let cookie_header_string = cookie_header.to_str().expect("Cookie should be a string");

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

    let claims = Jwt::verify(session_cookie?.to_owned());

    if claims.is_err() {
        return None;
    }

    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1 LIMIT 1",
        claims.unwrap().sub.unwrap().parse::<i32>().unwrap(),
    )
    .fetch_optional(pool)
    .await
    .expect("Failed to fetch user")
}
