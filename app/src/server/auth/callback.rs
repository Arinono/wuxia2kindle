use anyhow::{Result, Context};
use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Html},
};
use reqwest::header::SET_COOKIE;
use serde::Deserialize;
use sqlx::PgPool;

use super::{
    jwt::JWT,
    oauth::Service,
    user::User,
    discord::DiscordAuth,
    super::AppError,
};

#[derive(Debug, Deserialize)]
pub struct CallbackQueryParam {
    code: String,
}
pub async fn login_callback(
    State(pool): State<PgPool>,
    Path(service): Path<String>,
    Query(query): Query<CallbackQueryParam>,
) -> Result<impl IntoResponse, AppError> {
    let mut headers = HeaderMap::new();

    let code = &query.code;

    let service = match service.as_str() {
        "discord" => Service::Discord(DiscordAuth::new()),
        _ => {
            return Ok((headers, Html("Service not found")));
        }
    };

    let token = service.get_token(code).await.unwrap();

    let user = service.get_user(&token).await.unwrap();

    let db_user = match service {
        Service::Discord(_) => {
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE discord_id = $1 LIMIT 1",
                user.id,
            )
            .fetch_optional(&pool)
            .await?
        }
    };

    if db_user.is_none() {
        // only allowing one user (me) for now
        // the real impl would be to add the user
        return Ok((headers, Html("User not found")));
    }

    if user.avatar.is_some() {
        sqlx::query!(
            "UPDATE users SET avatar = $1 WHERE id = $2",
            user.avatar,
            db_user.as_ref().unwrap().id,
        ).execute(&pool).await?;
    }

    let jwt = JWT::new(db_user.unwrap().id.to_string());

    headers.insert(
        SET_COOKIE,
        jwt.to_string().parse().context("Failed to parse cookie")?,
    );

    Ok((headers, Html("<script>window.close()</script>")))
}
