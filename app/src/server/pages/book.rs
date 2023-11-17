use anyhow::Result;
use askama::Template;
use axum::extract::{State, Path};
use sqlx::PgPool;

use crate::server::{auth::user::User, books::Book, Error};

#[derive(Template)]
#[template(path = "book.html")]
pub struct NoCoverBook {
    id: i32,
    name: String,
    chapter_count: Option<i32>,
    author: Option<String>,
    translator: Option<String>,
}

pub async fn book(_user: User, State(pool): State<PgPool>, Path(book_id): Path<i32>) -> Result<NoCoverBook, Error> {
    let response = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1 LIMIT 1", book_id)
        .fetch_optional(&pool)
        .await?;

    let book = match response {
        Some(book) => NoCoverBook {
            id: book.id,
            name: book.name,
            chapter_count: book.chapter_count,
            author: book.author,
            translator: book.translator,
        },
        None => return Err(Error::NotFound("Book not found".to_owned())),
    };

    Ok(book)
}
