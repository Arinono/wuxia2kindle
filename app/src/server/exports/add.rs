use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use sqlx::PgPool;

use super::{AddToQueue, ExportKinds};

pub async fn add_to_queue(
    State(pool): State<PgPool>,
    Form(input): Form<AddToQueue>,
) -> impl IntoResponse {
    let export = ExportKinds::ChaptersRange {
        book_id: input.book_id,
        chapters: (input.from, input.to),
    };
    println!("Received export: {}", export);
    // todo check input validity, such as range start < end and stuff like this

    match sqlx::query!(
        "INSERT INTO exports (meta) VALUES ($1)",
        serde_json::to_value(export).unwrap(),
    )
    .execute(&pool)
    .await
    {
        Ok(_) => {
            (StatusCode::CREATED, Html("Export added to queue"))
        }
        Err(e) => {
            eprintln!("Error adding export to queue: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Error adding export to queue"),
            );
        }
    }
}
