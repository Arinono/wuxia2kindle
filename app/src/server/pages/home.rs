use anyhow::Result;
use askama::Template;

use crate::server::{auth::user::User, Error};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    title: String,
}

pub async fn home(user: Option<User>) -> Result<HomeTemplate, Error> {
    if user.is_none() {
        return Err(Error::Unauthenticated);
    }

    Ok(HomeTemplate {
        title: "Home | ".to_string(),
    })
}
