use crate::pool;
use axum::{
    error_handling::HandleErrorLayer,
    extract::State,
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{types::JsonValue, PgPool};
use std::{fmt::Display, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn start(port: u16, database_url: String) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wuxia2kindle=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = pool::mk_pool(database_url).await;

    // build our application with a route
    let app = Router::new()
        .route("/health", get(health))
        .route("/chapter", post(add_chapter))
        .route("/export", post(add_to_queue))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(Any),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(5))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(pool);

    tracing::debug!("Listening on 0.0.0.0:{port}");
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct AddChapter {
    book: String,
    name: String,
    content: String,
    number_in_book: i16,
}

impl Display for AddChapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} #{}", self.book, self.name, self.number_in_book)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AddToQueue {
    kind: ExportKinds,
}

impl Display for ExportKinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportKinds::ChaptersRange { book_id, chapters } => write!(
                f,
                "{}: Chapters from {} to {}",
                book_id, chapters.0, chapters.1
            ),
            ExportKinds::ChaptersList { book_id, chapters } => {
                write!(f, "{}: {} chapters", book_id, chapters.len())
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiRequest {
    AddChapter(AddChapter),
    AddToQueue(AddToQueue),
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiResponse {
    AddChapter { success: bool },
    AddToQueue { success: bool },
}

#[axum::debug_handler]
async fn add_chapter(
    State(pool): State<PgPool>,
    Json(input): Json<AddChapter>,
) -> impl IntoResponse {
    println!("Received chapter: {input}");

    let book: Option<Book> = {
        sqlx::query_as!(
            Book,
            "SELECT id, name, chapter_count FROM books b WHERE b.name = $1",
            input.book,
        )
        .fetch_optional(&pool)
        .await
        .unwrap()
    };

    match book {
        None => {
            println!("Inserting new book: {}", input.book);
            sqlx::query!("INSERT INTO books (name) VALUES ($1)", input.book)
                .execute(&pool)
                .await
                .unwrap();
        }
        Some(book) => {
            println!("Inserting new chapter: {}", input);
            if let Ok(_) = sqlx::query!(
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
            .await
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
        }
    }

    (
        StatusCode::CREATED,
        Json(ApiResponse::AddChapter { success: true }),
    )
}

async fn add_to_queue(
    State(pool): State<PgPool>,
    Json(input): Json<AddToQueue>,
) -> impl IntoResponse {
    println!("Received export: {}", input.kind);
    // todo check input validity, such as range start < end and stuff like this

    sqlx::query!(
        "INSERT INTO exports (meta) VALUES ($1)",
        input.kind.to_json(),
    )
    .execute(&pool)
    .await
    .unwrap();

    (
        StatusCode::CREATED,
        Json(ApiResponse::AddToQueue { success: true }),
    )
}

async fn health() -> &'static str {
    "Healthy!"
}

#[derive(Debug)]
struct Book {
    id: i32,
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    chapter_count: Option<i16>,
}

#[derive(Debug, Serialize, Clone)]
struct Chapter {
    id: i32,
    book_id: i32,
    name: String,
    content: String,
    number_in_book: i16,
    #[allow(dead_code)]
    processed: bool,
    #[allow(dead_code)]
    processed_at: Option<String>,
}

impl Display for Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) {} #{}",
            self.book_id, self.name, self.number_in_book
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum ExportKinds {
    Anthology,
    FullBook(i16),
    SingleChapter(i16),
    // will error if there is a blank spot in the range
    ChaptersRange { book_id: i16, chapters: (i16, i16) },
    ChaptersList { book_id: i16, chapters: Vec<i16> },
}

impl ExportKinds {
    fn to_json(&self) -> JsonValue {
        match self {
            ExportKinds::ChaptersRange { chapters, .. } => serde_json::to_value(self).unwrap(),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Export {
    id: i32,
    meta: ExportKinds,
    created_at: String,
    processing_started_at: Option<String>,
    processed_at: Option<String>,
    sent: bool,
    error: Option<String>,
}

enum ExportState {
    Created,
    Processing,
    Processed,
    Sent,
    Failed,
}

impl Display for ExportState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            ExportState::Sent => "sent",
            ExportState::Processed => "processed",
            ExportState::Processing => "processing",
            ExportState::Created => "created",
            ExportState::Failed => "failed",
        };

        write!(f, "{state}")
    }
}

impl Export {
    fn get_state(&self) -> ExportState {
        match self.error {
            Some(_) => ExportState::Failed,
            None => match self.sent {
                true => ExportState::Sent,
                false => match self.processed_at {
                    Some(_) => ExportState::Processed,
                    None => match self.processing_started_at {
                        Some(_) => ExportState::Processing,
                        None => ExportState::Created,
                    },
                },
            },
        }
    }
}

impl Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.get_state(), self.meta)
    }
}
