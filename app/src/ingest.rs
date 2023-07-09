use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, extract::State,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
pub async fn start(port: u16, database_url: String) {
    let pool = mk_pool(database_url).await;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/health", get(root))
        .route("/chapter", post(add_chapter))
        .with_state(pool);

    println!("Listening on 0.0.0.0:{port}");
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
        chapter_in_book: u8,
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
    chapter_in_book: u8,
}

#[axum::debug_handler]
async fn add_chapter(State(pool): State<PgPool>, Json(input): Json<AddChapter>) -> impl IntoResponse {
    let chapter = Chapter {
        book: input.book,
        name: input.name,
        content: input.content,
        chapter_in_book: input.chapter_in_book,
    };

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
    chapter_in_book: u8,
}
