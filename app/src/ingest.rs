use std::time::Duration;

use axum::{
    http::{StatusCode, HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
    Json, Router, extract::State, error_handling::HandleErrorLayer,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};
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

    let pool = mk_pool(database_url).await;

    // build our application with a route
    let app = Router::new()
        .route("/health", get(root))
        .route("/chapter", post(add_chapter))
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

#[derive(Debug, Serialize, Deserialize)]
enum ApiRequest {
    AddChapter {
        book: String,
        chapter: String,
        content: String,
        chapter_in_book: u16,
    },
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiResponse {
    AddChapter { success: bool },
}

#[derive(Debug, Deserialize)]
struct AddChapter {
    book: String,
    name: String,
    content: String,
    chapter_in_book: u16,
}

#[axum::debug_handler]
async fn add_chapter(State(pool): State<PgPool>, Json(input): Json<AddChapter>) -> impl IntoResponse {
    let chapter = Chapter {
        book: input.book,
        name: input.name,
        content: input.content,
        chapter_in_book: input.chapter_in_book,
    };
    println!("Received chapter: {chapter:#?}");

    #[derive(Debug)]
    struct Book {
        id: i32,
        name: String,
        chapter_count: Option<i16>,
    }

    let book: Option<Book> = {
        sqlx::query_as!(
            Book,
            "SELECT id, name, chapter_count FROM books b WHERE b.name = $1",
            &chapter.book,
        )
        .fetch_optional(&pool)
        .await
        .unwrap()
    };

    match book {
        None => {
            println!("Inserting new book: {}", chapter.book);
            sqlx::query!("INSERT INTO books (name) VALUES ($1)", &chapter.book)
                .execute(&pool)
                .await
                .unwrap();
        },
        Some (book) => {
            println!("{book:#?}");
            todo!();
        }
    }

    (StatusCode::CREATED, Json(chapter))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Healthy!"
}

async fn mk_pool(url: String) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();

    pool
}

#[derive(Debug, Serialize, Clone)]
struct Chapter {
    book: String,
    name: String,
    content: String,
    chapter_in_book: u16,
}
