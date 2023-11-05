use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response}, debug_handler,
};

use super::{discord::DiscordAuth, oauth::Service};

#[debug_handler]
pub async fn login(Path(service): Path<String>) -> impl IntoResponse {
    let service = match service.as_str() {
        "discord" => Service::Discord(DiscordAuth::new()),
        _ => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("HX-Reswap", "innerHTML")
                .header("Content-type", "text/plain")
                .body(format!("Service {} not found", service))
                .unwrap();
        }
    };

    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", service.authorize_url())
        .header("HX-Redirect", service.authorize_url())
        .body("".to_owned())
        .unwrap()
}
