use anyhow::Result;
use askama::Template;

use crate::server::{auth::AuthKind, Error};

#[derive(Template)]
#[template(path = "settings.html")]
pub struct SettingsTemplate {}

pub async fn settings(auth: AuthKind) -> Result<SettingsTemplate, Error> {
    if let Err(error) = auth.human() {
        return Err(error);
    }

    Ok(SettingsTemplate {})
}
