use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use models::book::Book;
use sqlx::PgPool;

use crate::server::{auth::AuthKind, Error};

use super::{AddChapter, Responses};

#[debug_handler]
pub async fn add_chapter(
    auth: AuthKind,
    State(pool): State<PgPool>,
    Json(input): Json<AddChapter>,
) -> Result<impl IntoResponse, Error> {
    if let Err(error) = auth.machine() {
        return Err(error);
    }
    println!("Received chapter: {input}");

    let mut o_book: Option<Book> = {
        sqlx::query_as!(Book, "SELECT * FROM books b WHERE b.name = $1", input.book)
            .fetch_optional(&pool)
            .await
            .unwrap()
    };

    if o_book.is_none() {
        println!("Inserting new book: {}", input.book);
        sqlx::query!(
            "INSERT INTO books (name, author, translator) VALUES ($1, $2, $3)",
            input.book,
            input.author,
            input.translator,
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    o_book = {
        sqlx::query_as!(Book, "SELECT * FROM books b WHERE b.name = $1", input.book,)
            .fetch_optional(&pool)
            .await
            .unwrap()
    };

    let book = o_book.expect("Book should exist");

    println!("Inserting new chapter: {}", input);
    if (sqlx::query!(
        "INSERT INTO chapters (
            book_id,
            name,
            content,
            number_in_book
            ) VALUES ($1, $2, $3, $4)",
        book.id,
        input.name,
        input.content,
        input.number_in_book,
    )
    .execute(&pool)
    .await)
        .is_ok()
    {
        let count = match book.chapter_count {
            None => 1,
            Some(c) => c + 1,
        };
        sqlx::query!(
            "UPDATE books
            SET chapter_count = $2
            WHERE id = $1",
            book.id,
            count,
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    Ok((
        StatusCode::CREATED,
        Json(Responses::AddChapter { success: true }),
    ))
}
