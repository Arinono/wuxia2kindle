use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;

use super::{
    Chapter,
    Responses::{Empty, GetChapter, GetChapters},
};

pub async fn get_chapters(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let chapters = sqlx::query_as!(
        Chapter,
        "SELECT * FROM chapters WHERE book_id = $1 ORDER BY number_in_book ASC",
        id
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    (StatusCode::OK, Json(GetChapters { data: chapters }))
}

pub async fn get_chapter(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let chapter = sqlx::query_as!(Chapter, "SELECT * FROM chapters WHERE id = $1", id)
        .fetch_optional(&pool)
        .await;

    match chapter {
        Err(e) => {
            println!("Error getting chapter: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Empty))
        }
        Ok(o_chapter) => match o_chapter {
            None => (StatusCode::NOT_FOUND, Json(Empty)),
            Some(chapter) => (StatusCode::OK, Json(GetChapter { data: chapter })),
        },
    }
}
