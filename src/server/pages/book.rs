use anyhow::Result;
use askama::Template;
use axum::extract::{Path, State};
use sqlx::PgPool;

use crate::server::{auth::AuthKind, Error};

struct NoCoverBook {
    id: i32,
    name: String,
    chapter_count: Option<i32>,
    author: Option<String>,
    translator: Option<String>,
}

#[derive(Clone)]
struct Chapter {
    id: i32,
    name: String,
    number: i32,
}

#[derive(Template)]
#[template(path = "book.html")]
pub struct BookAndChaptersTemplate {
    book: NoCoverBook,
    chapters: Vec<Chapter>,
    reverse: fn(Vec<Chapter>) -> Vec<Chapter>,
}

#[derive(Debug)]
struct BookAndChaptersQuery {
    id: i32,
    name: String,
    chapter_count: Option<i32>,
    author: Option<String>,
    translator: Option<String>,
    chapter_id: Option<i32>,
    chapter_name: Option<String>,
    chapter_number: i32,
}

pub async fn book(
    auth: AuthKind,
    State(pool): State<PgPool>,
    Path(book_id): Path<i32>,
) -> Result<BookAndChaptersTemplate, Error> {
    if let Err(error) = auth.human() {
        return Err(error);
    }

    let response = sqlx::query_as!(
        BookAndChaptersQuery,
        "
        SELECT
            b.id id,
            b.name name,
            b.chapter_count chapter_count,
            b.author author,
            b.translator translator,
            c.id chapter_id,
            c.name chapter_name,
            c.number_in_book chapter_number
        FROM chapters c
            LEFT JOIN books b ON b.id = c.book_id
        WHERE b.id = $1
        ORDER BY c.number_in_book ASC",
        book_id,
    )
    .fetch_all(&pool)
    .await?;

    if response.is_empty() {
        return Err(Error::NotFound("Book not found".to_owned()));
    }

    let raw_book = response.first().expect("cannot get book");
    let book = NoCoverBook {
        id: raw_book.id,
        name: raw_book.name.to_owned(),
        chapter_count: raw_book.chapter_count,
        author: raw_book.author.to_owned(),
        translator: raw_book.translator.to_owned(),
    };

    let chapters: Vec<Chapter> = response
        .iter()
        .filter_map(|chapter| {
            if chapter.chapter_id.is_some() {
                let name = match &chapter.chapter_name {
                    Some(name) => name.clone(),
                    None => "".to_owned(),
                };
                Some(Chapter {
                    id: chapter.chapter_id.expect("cannot get chapter id"),
                    name,
                    number: chapter.chapter_number,
                })
            } else {
                None
            }
        })
        .collect();

    let reverse = |chapters: Vec<Chapter>| {
        let mut rev_chapters = chapters.clone();
        rev_chapters.sort_by(|a, b| b.number.cmp(&a.number));
        rev_chapters
    };

    Ok(BookAndChaptersTemplate {
        book,
        chapters,
        reverse,
    })
}
