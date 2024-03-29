use std::collections::HashMap;

use axum::http::HeaderMap;
use serde::Deserialize;
use url::form_urlencoded::byte_serialize;

use self::super::super::Error;

use super::discord::DiscordAuth;

#[derive(Debug, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthUser {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
}

trait OAuth {
    fn authorize_url(&self) -> String;
    fn get_token_construct(&self, code: &str) -> (HeaderMap, HashMap<String, String>);
}

impl OAuth for DiscordAuth {
    fn authorize_url(&self) -> String {
        let redirect_uri = byte_serialize(self.redirect_uri.as_bytes()).collect::<String>();

        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=identify",
            self.base_authorize_url, self.client_id, redirect_uri
        )
    }

    fn get_token_construct(&self, code: &str) -> (HeaderMap, HashMap<String, String>) {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headers.insert(
            "Accept",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form = HashMap::new();
        form.insert("client_id".to_owned(), self.client_id.to_owned());
        form.insert("client_secret".to_owned(), self.client_secret.to_owned());
        form.insert("grant_type".to_owned(), "authorization_code".to_owned());
        form.insert("code".to_owned(), code.to_owned());
        form.insert("redirect_uri".to_owned(), self.redirect_uri.to_owned());

        (headers, form)
    }
}

#[derive(Debug)]
pub enum Service {
    Discord(DiscordAuth),
}

impl Service {
    pub fn authorize_url(&self) -> String {
        match self {
            Service::Discord(discord) => discord.authorize_url(),
        }
    }

    pub async fn get_token(&self, code: &str) -> Result<OAuthToken, Error> {
        match self {
            Service::Discord(discord) => {
                let (headers, form) = discord.get_token_construct(code);

                let client = reqwest::Client::new();
                let response = client
                    .post(&discord.token_url)
                    .headers(headers)
                    .form(&form)
                    .send()
                    .await?;

                let token: OAuthToken = response.json().await?;

                Ok(token)
            }
        }
    }

    pub async fn get_user(&self, token: &OAuthToken) -> Result<OAuthUser, Error> {
        match self {
            Service::Discord(_) => {
                let client = reqwest::Client::new();
                let response = client
                    .get("https://discord.com/api/users/@me")
                    .header("Authorization", format!("Bearer {}", token.access_token))
                    .send()
                    .await?;

                let mut user: OAuthUser = response.json().await?;

                user.avatar = match user.avatar {
                    Some(avatar) => Some(format!(
                        "https://cdn.discordapp.com/avatars/{}/{}.png",
                        user.id, avatar
                    )),
                    None => None,
                };

                Ok(user)
            }
        }
    }
}
