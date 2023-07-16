use std::time::Duration;

use sqlx::PgPool;
use tokio::time::interval;

use crate::{
    epub::Epub,
    models::{Book, Chapter, Export, ExportKinds},
    pool,
};

#[tokio::main]
pub async fn start(database_url: String) {
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        let pool = pool::mk_pool(database_url.clone()).await;
        tokio::spawn(async move {
            export(pool).await.close().await;
        });
    }
}

async fn export(pool: PgPool) -> PgPool {
    let exports: Vec<Export> = {
        sqlx::query_as!(
            Export,
            "SELECT * FROM exports WHERE processing_started_at IS NULL LIMIT 10",
        )
        .fetch_all(&pool)
        .await
        .unwrap()
    };

    for export in exports.into_iter() {
        sqlx::query!(
            "UPDATE exports 
            SET processing_started_at = CURRENT_TIMESTAMP
            WHERE id = $1",
            export.id,
        )
        .execute(&pool)
        .await
        .unwrap();

        if let Err(err) = process(export.clone(), &pool).await {
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
        } else {
            sqlx::query!(
                "UPDATE exports 
                SET processed_at = CURRENT_TIMESTAMP
                WHERE id = $1",
                export.id,
            )
            .execute(&pool)
            .await
            .unwrap();
        }
    }

    pool
}

async fn process(export: Export, pool: &PgPool) -> Result<(), String> {
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
            }
        }
        _ => todo!(),
    }

    Ok(())
}
