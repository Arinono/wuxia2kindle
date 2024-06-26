use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use models::book::Book;
use sqlx::PgPool;

use super::UpdateBook;

#[allow(dead_code)]
#[debug_handler]
pub async fn update_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateBook>,
) -> impl IntoResponse {
    if let Ok(Some(mut book)) = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
    {
        if let Some(name) = input.name {
            book.name = name;
        }

        if let Some(cover) = input.cover {
            book.cover = Some(cover);
        }

        let res = sqlx::query!(
            "UPDATE books SET name = $2, cover = $3 WHERE id = $1",
            id,
            book.name,
            book.cover
        )
        .execute(&pool)
        .await;

        if let Err(e) = res {
            println!("Error updating book: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, ());
        }

        return (StatusCode::ACCEPTED, ());
    }

    (StatusCode::NOT_FOUND, ())
}
