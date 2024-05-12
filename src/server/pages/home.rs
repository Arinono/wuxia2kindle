use anyhow::Result;
use askama::Template;

use crate::server::{auth::AuthKind, Error};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {}

pub async fn home(auth: AuthKind) -> Result<HomeTemplate, Error> {
    if let Err(error) = auth.human() {
        return Err(error);
    }

    Ok(HomeTemplate {})
}
