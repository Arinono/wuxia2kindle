#[derive(Debug)]
pub struct DiscordAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub base_authorize_url: String,
    pub token_url: String,
}

impl DiscordAuth {
    pub fn new() -> Self {
        let client_id = std::env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set");
        let client_secret =
            std::env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set");
        let redirect_uri =
            std::env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI must be set");

        Self {
            client_id,
            client_secret,
            redirect_uri,
            base_authorize_url: "https://discord.com/api/oauth2/authorize".to_owned(),
            token_url: "https://discord.com/api/oauth2/token".to_owned(),
        }
    }
}
