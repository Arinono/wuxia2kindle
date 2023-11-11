pub mod auth;
pub mod books;
pub mod chapters;
pub mod components;
pub mod exports;
pub mod files;
pub mod health;

use self::{
    auth::{
        callback::login_callback, cookie::get_cookie, login::login, logout::logout, user::User,
    },
    books::{
        get::{get_book, get_books},
        update::update_book,
    },
    chapters::{
        add::add_chapter,
        get::{get_chapter, get_chapters},
    },
    components::{avatar::avatar, books::books, cover::cover},
    exports::add::add_to_queue,
    files::{index, login_page, static_path},
    health::health,
};
use super::pool;
use axum::{
    async_trait,
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, FromRef, FromRequestParts},
    http::{
        header::{self},
        request::Parts,
        HeaderMap, HeaderValue, Method, StatusCode,
    },
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use include_dir::{include_dir, Dir};
use sqlx::PgPool;
use std::time::Duration;
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
        // statics
        .route("/login", get(login_page))
        .route("/login.html", get(login_page))
        .route("/", get(index))
        .route("/*path", get(static_path))
        // misc
        .route("/health", get(health))
        // auth
        .route("/auth/:service/login", get(login))
        .route("/logout", get(logout))
        .route("/auth/:service/callback", get(login_callback))
        // new
        .route("/avatar", get(avatar))
        .route("/books", get(books))
        .route("/book/:id/cover", get(cover))
        // legacy
        .route("/chapter", post(add_chapter))
        .route("/chapter/:id", get(get_chapter))
        // .route("/books", get(get_books))
        .route("/book/:id", get(get_book).patch(update_book))
        .route("/book/:id/chapters", get(get_chapters))
        .route("/export", post(add_to_queue))
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(vec![
                    domain.parse::<HeaderValue>().unwrap(),
                    "https://www.wuxiaworld.com".parse::<HeaderValue>().unwrap(),
                ])
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

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> axum::response::Response {
        Redirect::temporary("/login").into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        let header = parts.headers.get("cookie").ok_or(AuthRedirect)?;

        let mut headers = HeaderMap::new();
        headers.insert("cookie", header.clone());

        let user = get_cookie(&headers, &pool).await.ok_or(AuthRedirect)?;

        Ok(user)
    }
}
