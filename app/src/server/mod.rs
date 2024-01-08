pub mod auth;
pub mod books;
pub mod chapters;
pub mod exports;
pub mod health;
pub mod pages;

use self::{
    auth::{callback::login_callback, cookie::get_cookie, logout::logout, user::User},
    chapters::{add::add_chapter, get::get_chapters},
    exports::add::add_to_queue,
    health::health,
};
use super::pool;
use askama::Template;
use axum::{
    async_trait,
    body::{self, Empty},
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, FromRef, FromRequestParts},
    http::{
        header::{self},
        request::Parts,
        HeaderMap, HeaderValue, Method, Response, StatusCode,
    },
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const ADDRESS: &str = "0.0.0.0";

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
        .route("/login", get(pages::login::login))
        .route("/", get(pages::home::home))
        .route("/avatar", get(pages::partials::avatar::avatar))
        .route("/books", get(pages::partials::books::books))
        .route("/book/:id", get(pages::book::book))
        .route("/book/:id/cover", get(pages::partials::cover::cover))
        .route("/chapter/:id", get(pages::chapter::chapter))
        .route("/settings", get(pages::settings::settings))
        .route("/token", get(pages::partials::token::get_token))
        // misc
        .route("/health", get(health))
        // auth
        .route("/auth/:service/login", get(auth::login::login))
        .route("/logout", get(logout))
        .route("/auth/:service/callback", get(login_callback))
        // new
        // legacy
        .route("/chapter", post(add_chapter))
        // .route("/chapter/:id", get(get_chapter))
        // .route("/books", get(get_books))
        // .route("/book/:id", get(get_book).patch(update_book))
        .route("/book/:id/chapters", get(get_chapters))
        .route("/export", post(add_to_queue))
        .route("/*catchall", get(not_found))
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

    tracing::debug!("Listening on:\nhttp://{ADDRESS}:{port}");
    axum::Server::bind(&format!("{ADDRESS}:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn not_found() -> Result<Html<String>, Error> {
    Err(Error::NotFound("Page not found".to_string()))
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
pub enum Error {
    NotFound(String),
    UserAlreadyLoggedIn,
    Unauthenticated,
    AppError(anyhow::Error),
    AuthRedirect,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {
    message: String,
}

#[derive(Template)]
#[template(path = "500.html")]
pub struct AppErrorTemplate {
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> askama_axum::Response {
        match self {
            Self::NotFound(message) => {
                let body = NotFoundTemplate { message }.render().unwrap();

                (StatusCode::NOT_FOUND, Html(body)).into_response()
            }
            Self::UserAlreadyLoggedIn => Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/")
                .body(body::boxed(Empty::new()))
                .unwrap(),
            Self::Unauthenticated => Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/login")
                .body(body::boxed(Empty::new()))
                .unwrap(),
            Self::AppError(e) => {
                tracing::error!("Application error: {:#}", e);
                let body = AppErrorTemplate {
                    message: e.to_string(),
                }
                .render()
                .unwrap();

                (StatusCode::INTERNAL_SERVER_ERROR, Html(body)).into_response()
            }
            Self::AuthRedirect => Redirect::temporary("/login").into_response(),
        }
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self::AppError(e.into())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        let header = parts.headers.get("cookie").ok_or(Error::AuthRedirect)?;

        let mut headers = HeaderMap::new();
        headers.insert("cookie", header.clone());

        let user = get_cookie(&headers, &pool)
            .await
            .ok_or(Error::AuthRedirect)?;

        Ok(user)
    }
}
