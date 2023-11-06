use askama::Template;
use askama_axum::IntoResponse;

use super::super::super::server::auth::user::User;

#[derive(Template)]
#[template(path = "avatar.html")]
#[allow(dead_code)]
struct Avatar {
    pub name: String,
    pub avatar: String,
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
