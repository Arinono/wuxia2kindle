use anyhow::Result;
use askama::Template;

use crate::server::{auth::user::User, Error};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    title: String,
}

pub async fn login(user: Option<User>) -> Result<LoginTemplate, Error> {
    if user.is_some() {
        return Err(Error::UserAlreadyLoggedIn);
    }

    Ok(LoginTemplate {
        title: "Login | ".to_string(),
    })
}
