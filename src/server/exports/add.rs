use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use sqlx::PgPool;

use reqwest::multipart;
use serde::Serialize;

use crate::{
    env::Environment,
    server::{
        books::Book,
        chapters::Chapter,
        exports::{epub::Epub, Export},
    },
};

use super::{AddToQueue, ExportKinds};

#[derive(Debug, Serialize)]
struct Attachement {
    file: String,
    description: String,
    filename: String,
}

#[derive(Debug, Serialize)]
struct EmbedField {
    name: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct Embed {
    title: String,
    #[serde(rename = "type")]
    r#type: String,
    description: String,
    color: u32,
    fields: Vec<EmbedField>,
}

#[derive(Debug, Serialize)]
struct Message {
    content: String,
    embeds: Vec<Embed>,
}

struct MessageBuilder {
    content: String,
    book_name: String,
    from: i32,
    to: i32,
}

impl MessageBuilder {
    fn new() -> Self {
        Self {
            content: "Your book is ready!".to_owned(),
            book_name: "".to_owned(),
            from: 0,
            to: 0,
        }
    }

    fn book_name(mut self, book_name: String) -> Self {
        self.book_name = book_name;
        self
    }

    fn from(mut self, from: i32) -> Self {
        self.from = from;
        self
    }

    fn to(mut self, to: i32) -> Self {
        self.to = to;
        self
    }

    fn build(self) -> Message {
        Message {
            content: self.content,
            embeds: vec![Embed {
                title: self.book_name,
                r#type: "file".to_owned(),
                description: format!("From chapter {} to chapter {}", self.from, self.to),
                color: 0x91288a,
                fields: vec![],
            }],
        }
    }
}

async fn run_export(pool: PgPool, export: Export, webhook_url: &String) {
    sqlx::query!(
        "UPDATE exports 
        SET processing_started_at = CURRENT_TIMESTAMP
        WHERE id = $1",
        export.id,
    )
    .execute(&pool)
    .await
    .unwrap();

    match process(export.clone(), &pool).await {
        Err(err) => {
            sqlx::query!(
                "UPDATE exports 
                SET processed_at = CURRENT_TIMESTAMP,
                    error = $2
                WHERE id = $1",
                export.id,
                err,
            )
            .execute(&pool)
            .await
            .unwrap();
        }
        Ok(path) => {
            sqlx::query!(
                "UPDATE exports 
                SET processed_at = CURRENT_TIMESTAMP
                WHERE id = $1",
                export.id,
            )
            .execute(&pool)
            .await
            .unwrap();

            let (book_id, from, to) = match export.meta {
                ExportKinds::ChaptersRange { book_id, chapters } => {
                    (book_id, chapters.0, chapters.1)
                }
                _ => todo!(),
            };
            let book = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", book_id)
                .fetch_one(&pool)
                .await
                .expect("book not found");

            let message = MessageBuilder::new()
                .book_name(book.name)
                .from(from)
                .to(to)
                .build();

            let filebody = std::fs::read(&path).unwrap();
            let file_part = multipart::Part::bytes(filebody)
                .file_name("book.epub")
                .mime_str("application/epub+zip")
                .unwrap();
            let json_part = multipart::Part::text(serde_json::to_string(&message).unwrap());
            let form = reqwest::multipart::Form::new()
                .part("book.epub", file_part)
                .part("payload_json", json_part);

            println!("Sending epub");
            let res = reqwest::Client::new()
                .post(webhook_url)
                .multipart(form)
                .send()
                .await
                .unwrap();

            if res.status().is_success() {
                println!("Epub sent");
                sqlx::query!(
                    "UPDATE exports
                    SET sent = true
                    WHERE id = $1",
                    export.id,
                )
                .execute(&pool)
                .await
                .unwrap();
            } else {
                println!("Epub not sent");
            }
        }
    }
}

async fn process(export: Export, pool: &PgPool) -> Result<String, String> {
    println!("Processing export {}", export.id);

    match export.meta {
        ExportKinds::ChaptersRange { book_id, chapters } => {
            let db_chapters: Vec<Chapter> = {
                sqlx::query_as!(
                    Chapter,
                    "SELECT * FROM chapters
                    WHERE book_id = $1
                       AND number_in_book >= $2
                       AND number_in_book <= $3
                    ORDER BY number_in_book ASC",
                    book_id,
                    chapters.0,
                    chapters.1,
                )
                .fetch_all(pool)
                .await
                .unwrap()
            };

            let o_book: Option<Book> = {
                sqlx::query_as!(
                    Book,
                    "SELECT *
                    FROM books
                    WHERE id = $1",
                    book_id,
                )
                .fetch_optional(pool)
                .await
                .unwrap()
            };

            if let Some(book) = o_book.clone() {
                let epub = Epub {
                    title: book.name,
                    author: book.author,
                    translator: book.translator,
                    cover: book.cover,
                    chapters: db_chapters
                        .into_iter()
                        .map(|c| (c.name, c.content))
                        .collect::<Vec<(String, String)>>(),
                };

                let filepath = epub.generate().unwrap();

                println!("Epub generated at: {filepath}");
                return Ok(filepath);
            }

            Err("book not found ?".to_owned())
        }
        _ => todo!(),
    }
}
pub async fn add_to_queue(
    State(pool): State<PgPool>,
    State(env): State<Environment>,
    Form(input): Form<AddToQueue>,
) -> impl IntoResponse {
    let export = ExportKinds::ChaptersRange {
        book_id: input.book_id,
        chapters: (input.from, input.to),
    };
    println!("Received export: {}", export);
    // todo check input validity, such as range start < end and stuff like this

    match sqlx::query_as!(
        Export,
        "INSERT INTO exports (meta) VALUES ($1) RETURNING *",
        serde_json::to_value(export).unwrap(),
    )
    .fetch_one(&pool)
    .await
    {
        Ok(export) => {
            tokio::spawn(async move {
                run_export(pool, export, &env.discord_webhook).await;
            });
            (StatusCode::CREATED, Html("Export added to queue"))
        }
        Err(e) => {
            eprintln!("Error adding export to queue: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Error adding export to queue"),
            )
        }
    }
}
