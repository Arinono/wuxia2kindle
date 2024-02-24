use anyhow::Result;
use askama::Template;
use axum::extract::State;
use sqlx::PgPool;

use crate::server::{auth::user::User, books::Book, Error};

pub struct MinimalBook {
    name: String,
    id: i32,
}

#[derive(Template)]
#[template(path = "partials/books.html")]
pub struct Books {
    books: Vec<MinimalBook>,
}

pub async fn books(_user: User, State(pool): State<PgPool>) -> Result<Books, Error> {
    let response = sqlx::query_as!(Book, "SELECT * FROM books")
        .fetch_all(&pool)
        .await?;

    let books = response
        .into_iter()
        .map(|book| MinimalBook {
            name: book.name.clone(),
            id: book.id,
        })
        .collect();

    Ok(Books { books })
}
