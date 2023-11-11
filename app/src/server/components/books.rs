use anyhow::Result;
use askama::Template;
use axum::extract::State;
use sqlx::PgPool;

use crate::server::{auth::user::User, books::Book, AppError};

pub struct PartialBook {
    name: String,
    id: i32,
}

#[derive(Template)]
#[template(path = "books.html")]
pub struct Books {
    books: Vec<PartialBook>,
}

pub async fn books(_user: User, State(pool): State<PgPool>) -> Result<Books, AppError> {
    let response = sqlx::query_as!(Book, "SELECT * FROM books")
        .fetch_all(&pool)
        .await?;

    let books = response
        .into_iter()
        .map(|book| PartialBook {
            name: book.name.clone(),
            id: book.id,
        })
        .collect();

    Ok(Books { books })
}
