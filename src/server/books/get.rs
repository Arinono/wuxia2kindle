use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::PgPool;

use super::{
    Book,
    Responses::{Empty, GetBook, GetBooks},
};

#[allow(dead_code)]
#[debug_handler]
pub async fn get_book(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let response = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", id)
        .fetch_optional(&pool)
        .await;

    match response {
        Err(e) => {
            println!("Error getting book: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Empty))
        }
        Ok(o_book) => match o_book {
            Some(book) => (StatusCode::OK, Json(GetBook { data: book })),
            None => (StatusCode::NOT_FOUND, Json(Empty)),
        },
    }
}

#[allow(dead_code)]
#[debug_handler]
pub async fn get_books(State(pool): State<PgPool>) -> impl IntoResponse {
    let books = sqlx::query_as!(Book, "SELECT * FROM books ORDER BY name")
        .fetch_all(&pool)
        .await;

    match books {
        Err(e) => {
            println!("Error getting books: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Empty))
        }
        Ok(books) => (StatusCode::OK, Json(GetBooks { data: books })),
    }
}
