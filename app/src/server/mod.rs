pub mod auth;
pub mod books;
pub mod chapters;
pub mod exports;

use super::pool;
use self::{
    auth::{callback::login_callback, cookie::get_cookie, login::login},
    books::{
        Book,
        get::{get_book, get_books},
        update::update_book,
    },
    chapters::Chapter,
    exports::ExportKinds,
};
use axum::{
    body::{self, Empty, Full},
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, FromRef, Path, State},
    http::{
        header::{self},
        HeaderMap, HeaderValue, Method, Response, StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{fmt::Display, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/static/src");

#[tokio::main]
pub async fn start(port: u16, database_url: String) {
    let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wuxia2kindle=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = pool::mk_pool(database_url).await;
    let app_state = AppState { pool };

    // build our application with a route
    let app = Router::new()
        .route("/*path", get(static_path))
        .route("/", get(index))
        .route("/health", get(health))
        .route("/auth/:service/login", get(login))
        .route("/auth/:service/callback", get(login_callback))
        .route("/chapter", post(add_chapter))
        .route("/chapter/:id", get(get_chapter))
        .route("/books", get(get_books))
        .route("/book/:id", get(get_book).patch(update_book))
        .route("/book/:id/chapters", get(get_chapters))
        .route("/export", post(add_to_queue))
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(domain.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PATCH])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT, header::COOKIE]),
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
        .with_state(app_state);

    tracing::debug!("Listening on:\nhttp://localhost:{port}");
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

async fn index(headers: HeaderMap, State(pool): State<PgPool>) -> impl IntoResponse {
    let user = get_cookie(&headers, &pool).await;
    println!("User: {:?}", user);
    if user.is_none() {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .body(body::boxed(Full::from(
                STATIC_DIR.get_file("login.html").unwrap().contents(),
            )))
            .unwrap();
    }

    let index = STATIC_DIR
        .get_file("index.html")
        .expect("index.html should exist");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .body(body::boxed(Full::from(index.contents())))
        .unwrap()
}

fn get_file(file: &str) -> impl IntoResponse {
    let path = file.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    let file = STATIC_DIR.get_file(path);
    match file {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    get_file(&path)
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
}

#[derive(Debug, Serialize, Deserialize)]
enum ApiResponse {
    AddChapter { success: bool },
    AddToQueue { success: bool },
    GetChapters { data: Vec<Chapter> },
    GetChapter { data: Option<Chapter> },
}

#[axum::debug_handler]
async fn add_chapter(
    State(pool): State<PgPool>,
    Json(input): Json<AddChapter>,
) -> impl IntoResponse {
    println!("Received chapter: {input}");

    let mut o_book: Option<Book> = {
        sqlx::query_as!(Book, "SELECT * FROM books b WHERE b.name = $1", input.book,)
            .fetch_optional(&pool)
            .await
            .unwrap()
    };

    if o_book.is_none() {
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

    o_book = {
        sqlx::query_as!(Book, "SELECT * FROM books b WHERE b.name = $1", input.book,)
            .fetch_optional(&pool)
            .await
            .unwrap()
    };

    let book = o_book.expect("Book should exist");

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
        .await;

    match chapter {
        Err(e) => {
            println!("Error getting chapter: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::GetChapter { data: None }),
            )
        }
        Ok(o_chapter) => (
            StatusCode::OK,
            Json(ApiResponse::GetChapter { data: o_chapter }),
        ),
    }
}

async fn health() -> &'static str {
    "Healthy!"
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Application error: {:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
