use askama::Template;
use askama_axum::IntoResponse;

use crate::server::auth::user::User;

#[derive(Template)]
#[template(path = "partials/avatar.html")]
pub struct Avatar {
    name: String,
    avatar: String,
}

pub async fn avatar(user: User) -> impl IntoResponse {
    let avatar = match user.avatar {
        Some(avatar) => avatar,
        None => format!(
            "https://ui-avatars.com/api/?background=0D8ABC&color=fff&name={}",
            user.username
        ),
    };

    Avatar {
        name: user.username,
        avatar,
    }
}
