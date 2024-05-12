use anyhow::Result;
use askama::Template;
use axum::extract::{Path, State};
use sqlx::PgPool;

use crate::server::{auth::AuthKind, Error};

#[derive(Template)]
#[template(path = "partials/cover.html")]
pub struct Cover {
    name: String,
    cover: Option<String>,
}

pub async fn cover(
    auth: AuthKind,
    State(pool): State<PgPool>,
    Path(book_id): Path<i32>,
) -> Result<Cover, Error> {
    if let Err(error) = auth.human() {
        return Err(error);
    }

    let response = sqlx::query_as!(
        Cover,
        "SELECT name, cover from books where id = $1 LIMIT 1",
        book_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Cover {
        name: response.name,
        cover: response.cover,
    })
}
