use anyhow::Result;
use askama::Template;

use crate::server::{auth::user::User, Unauthenticated};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

pub async fn home(user: Option<User>) -> Result<HomeTemplate, Unauthenticated> {
    if user.is_none() {
        return Err(Unauthenticated);
    }

    Ok(HomeTemplate)
}
