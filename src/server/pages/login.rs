use anyhow::Result;
use askama::Template;

use crate::server::{auth::AuthKind, Error};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    title: String,
}

pub async fn login(auth: Option<AuthKind>) -> Result<LoginTemplate, Error> {
    if auth.is_some() {
        return Err(Error::UserAlreadyLoggedIn);
    }

    Ok(LoginTemplate {
        title: "Login | ".to_string(),
    })
}
