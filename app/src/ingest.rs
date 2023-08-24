use crate::{
    models::{Book, Chapter, ExportKinds},
    pool,
};
use axum::{
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, Path, State},
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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
        .route("/chapter/:id", get(get_chapter))
        .route("/books", get(get_books))
        .route("/book/:id", get(get_book).patch(update_book))
        .route("/book/:id/chapters", get(get_chapters))
        .route("/export", post(add_to_queue))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH])
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
        .layer(DefaultBodyLimit::max(5_242_880))
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
    number_in_book: i32,
    author: Option<String>,
    translator: Option<String>,
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

#[derive(Debug, Deserialize, Serialize)]
struct UpdateBook {
    pub name: Option<String>,
    pub cover: Option<String>,
}

impl Display for ExportKinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportKinds::ChaptersRange { book_id, chapters } => write!(
                f,
                "{}: Chapters from {} to {}",
                book_id, chapters.0, chapters.1
            ),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiRequest {
    AddChapter(AddChapter),
    AddToQueue(AddToQueue),
    GetBooks,
    UpdateBook(UpdateBook),
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiResponse {
    AddChapter { success: bool },
    AddToQueue { success: bool },
    GetBooks { data: Vec<Book> },
    GetChapters { data: Vec<Chapter> },
    GetBook { data: Option<Book> },
    GetChapter { data: Option<Chapter> },
}

#[axum::debug_handler]
async fn add_chapter(
    State(pool): State<PgPool>,
    Json(input): Json<AddChapter>,
) -> impl IntoResponse {
    println!("Received chapter: {input}");

    let book: Option<Book> = {
        sqlx::query_as!(Book, "SELECT * FROM books b WHERE b.name = $1", input.book,)
            .fetch_optional(&pool)
            .await
            .unwrap()
    };

    match book {
        None => {
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
        Some(book) => {
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
        serde_json::to_value(input.kind).unwrap(),
    )
    .execute(&pool)
    .await
    .unwrap();

    (
        StatusCode::CREATED,
        Json(ApiResponse::AddToQueue { success: true }),
    )
}

async fn get_books(State(pool): State<PgPool>) -> impl IntoResponse {
    let books = sqlx::query_as!(Book, "SELECT * FROM books ORDER BY name",)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(ApiResponse::GetBooks { data: books }))
}

async fn get_chapters(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let chapters = sqlx::query_as!(
        Chapter,
        "SELECT * FROM chapters WHERE book_id = $1 ORDER BY number_in_book ASC",
        id
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    (
        StatusCode::OK,
        Json(ApiResponse::GetChapters { data: chapters }),
    )
}

async fn get_chapter(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let chapter = sqlx::query_as!(Chapter, "SELECT * FROM chapters WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    (
        StatusCode::OK,
        Json(ApiResponse::GetChapter { data: chapter }),
    )
}

async fn get_book(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let book = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(ApiResponse::GetBook { data: book }))
}

async fn update_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateBook>,
) -> impl IntoResponse {
    if let Ok(o_book) = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
    {
        if let Some(mut book) = o_book {
            if let Some(name) = input.name {
                book.name = name;
            }

            if let Some(cover) = input.cover {
                book.cover = Some(cover);
            }

            let res = sqlx::query!(
                "UPDATE books SET name = $2, cover = $3 WHERE id = $1",
                id,
                book.name,
                book.cover
            )
            .execute(&pool)
            .await;

            if let Err(e) = res {
                println!("Error updating book: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, ());
            }

            return (StatusCode::ACCEPTED, ());
        }
    }

    (StatusCode::NOT_FOUND, ())
}

async fn health() -> &'static str {
    "Healthy!"
}
