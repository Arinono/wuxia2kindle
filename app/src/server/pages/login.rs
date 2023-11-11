use anyhow::Result;
use askama::Template;

use crate::server::{auth::user::User, UserAlreadyLoggedIn};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

pub async fn login(user: Option<User>) -> Result<LoginTemplate, UserAlreadyLoggedIn> {
    if user.is_some() {
        return Err(UserAlreadyLoggedIn);
    }

    Ok(LoginTemplate)
}
