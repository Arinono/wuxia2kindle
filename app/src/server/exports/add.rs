use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use super::{AddToQueue, Responses};

pub async fn add_to_queue(
    State(pool): State<PgPool>,
    Json(input): Json<AddToQueue>,
) -> impl IntoResponse {
    println!("Received export: {}", input.kind);
    // todo check input validity, such as range start < end and stuff like this

    sqlx::query!(
        "INSERT INTO exports (meta) VALUES ($1)",
        serde_json::to_value(input.kind).unwrap(),
    )
    .execute(&pool)
    .await
    .unwrap();

    (
        StatusCode::CREATED,
        Json(Responses::AddToQueue { success: true }),
    )
}
