use anyhow::Result;
use askama::Template;

use crate::server::{auth::user::User, Error};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {}

pub async fn home(_user: User) -> Result<HomeTemplate, Error> {
    Ok(HomeTemplate {})
}
