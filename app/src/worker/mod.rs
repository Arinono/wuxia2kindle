mod epub;

use std::time::Duration;

use epub::Epub;

use lettre::{
    message::{header::ContentType, Attachment},
    Message, SmtpTransport, Transport,
};
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

#[tokio::main]
pub async fn start(database_url: String, mailer: SmtpTransport, send_to: String, from: String) {
    let mut interval = interval(Duration::from_secs(60 * 60 * 6));

    loop {
        interval.tick().await;
        let pool = pool::mk_pool(database_url.clone()).await;
        export(pool, &mailer, &send_to, &from)
            .await
            .close()
            .await;
    }
}

async fn export(pool: PgPool, mailer: &SmtpTransport, send_to: &String, from: &String) -> PgPool {
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

                let from = format!("Wuxia2Kindle <{}>", from);
                let filebody = std::fs::read(&path).unwrap();
                let content_type = ContentType::parse("application/epub+zip").unwrap();
                let attachement =
                    Attachment::new("book.epub".to_owned()).body(filebody, content_type);
                let email = Message::builder()
                    .from(from.parse().unwrap())
                    .reply_to(from.parse().unwrap())
                    .to(format!("Kindle <{}>", send_to).parse().unwrap())
                    .singlepart(attachement)
                    .unwrap();

                println!("Sending email");
                match &mailer.send(&email) {
                    Ok(_) => {
                        println!("Sent {path}");
                        sqlx::query!(
                            "UPDATE exports 
                            SET sent = true
                            WHERE id = $1",
                            export.id,
                        )
                        .execute(&pool)
                        .await
                        .unwrap();
                    }
                    Err(e) => panic!("could not send email: {:?}", e),
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
