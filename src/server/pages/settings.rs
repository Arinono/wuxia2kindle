use anyhow::Result;
use askama::Template;

use crate::server::{auth::user::User, Error};

#[derive(Template)]
#[template(path = "settings.html")]
pub struct SettingsTemplate {}

pub async fn settings(_user: User) -> Result<SettingsTemplate, Error> {
    Ok(SettingsTemplate {})
}
