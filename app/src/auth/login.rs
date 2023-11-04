use axum::{extract::Path, response::Response};
use reqwest::StatusCode;

use super::{discord::DiscordAuth, oauth::Service};

pub async fn login(Path(service): Path<String>) -> Response<String> {
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
