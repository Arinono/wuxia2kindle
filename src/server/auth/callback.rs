use anyhow::{Context, Result};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use models::{
    repository::{Repository, RepositoryError},
    user::User,
};
use reqwest::header::SET_COOKIE;
use serde::Deserialize;
use sqlx::PgPool;

use super::{super::Error, discord::DiscordAuth, jwt::Jwt, oauth::Service};

#[derive(Debug, Deserialize)]
pub struct CallbackQueryParam {
    code: String,
}

#[debug_handler]
pub async fn login_callback(
    State(pool): State<PgPool>,
    Path(service): Path<String>,
    Query(query): Query<CallbackQueryParam>,
) -> Result<impl IntoResponse, Error> {
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

    let maybe_db_user = match service {
        Service::Discord(_) => User::get_by_discord_id(&pool, user.id).await,
    };

    let mut db_user = match maybe_db_user {
        Ok(user) => user,
        Err(error) => match error {
            // only allowing one user (me) for now
            // the real impl would be to add the user
            RepositoryError::NotFound => return Ok((headers, Html("User not found"))),
            _ => return Err(Error::AppError(anyhow::anyhow!("Failed to get user"))),
        },
    };

    if user.avatar.is_some() {
        db_user.avatar = user.avatar;
        if let Err(error) = db_user.update(&pool).await {
            match error {
                RepositoryError::NotFound => return Ok((headers, Html("User not found"))),
                _ => return Err(Error::AppError(anyhow::anyhow!("Failed to update user"))),
            }
        }
    }

    let jwt = Jwt::new(db_user.id.to_string());

    headers.insert(
        SET_COOKIE,
        jwt.to_string().parse().context("Failed to parse cookie")?,
    );

    Ok((headers, Html("<script>window.close()</script>")))
}
