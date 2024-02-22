mod epub;

use std::time::Duration;

use epub::Epub;

use reqwest::multipart;
use serde::Serialize;
use sqlx::PgPool;
use tokio::time::interval;

use super::{
    pool,
    server::{
        books::Book,
        chapters::Chapter,
        exports::{Export, ExportKinds},
    },
};

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

#[tokio::main]
pub async fn start(database_url: String, webhook_url: String) {
    let mut interval = interval(Duration::from_secs(60 * 60 * 6));

    loop {
        interval.tick().await;
        let pool = pool::mk_pool(database_url.clone()).await;
        export(pool, &webhook_url).await.close().await;
    }
}

async fn export(pool: PgPool, webhook_url: &String) -> PgPool {
    let exports: Vec<Export> = {
        sqlx::query_as!(
            Export,
            "SELECT * FROM exports WHERE processing_started_at IS NULL LIMIT 10",
        )
        .fetch_all(&pool)
        .await
        .unwrap()
    };
    println!("Processing {} exports", exports.len());

    for export in exports.clone().into_iter() {
        sqlx::query!(
            "UPDATE exports 
            SET processing_started_at = CURRENT_TIMESTAMP
            WHERE id = $1",
            export.id,
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    for export in exports.into_iter() {
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
                let book = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", book_id,)
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
                } else {
                    println!("Epub not sent");
                }
            }
        }
    }

    pool
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
