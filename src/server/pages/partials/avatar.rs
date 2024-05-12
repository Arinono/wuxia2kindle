use askama::Template;
use askama_axum::IntoResponse;

use crate::server::{auth::AuthKind, Error};

#[derive(Template)]
#[template(path = "partials/avatar.html")]
pub struct Avatar {
    name: String,
    avatar: String,
}

pub async fn avatar(auth: AuthKind) -> Result<impl IntoResponse, Error> {
    let user = match auth.human() {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    let avatar = match &user.avatar {
        Some(avatar) => avatar.clone(),
        None => format!(
            "https://ui-avatars.com/api/?background=0D8ABC&color=fff&name={}",
            user.username
        ),
    };

    Ok(Avatar {
        name: user.username.clone(),
        avatar,
    })
}
