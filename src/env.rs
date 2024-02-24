use std::env::var;

#[derive(Debug, Clone)]
pub struct Environment {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub domain: String,
    pub salt: String,
    pub discord_webhook: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_redirect_uri: String,
}

impl Environment {
    pub fn new() -> Self {
        let port = var("PORT").unwrap_or("3000".to_owned());
        let database_url =
            var("DATABASE_URL").unwrap_or("postgres://localhost:5432/wuxia2kindle".to_owned());
        let jwt_secret = var("JWT_SECRET").expect("JWT_SECRET must be set");
        let domain = var("DOMAIN").expect("DOMAIN must be set");
        let salt = var("SALT").expect("SALT must be set");
        let discord_webhook = var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK must be set");
        let discord_client_id = var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set");
        let discord_client_secret =
            var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set");
        let discord_redirect_uri =
            var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI must be set");

        Self {
            port: port.parse().expect("PORT must be a number"),
            database_url,
            jwt_secret,
            domain,
            salt,
            discord_webhook,
            discord_client_id,
            discord_client_secret,
            discord_redirect_uri,
        }
    }
}
