use std::fmt::Display;

use jsonwebtoken::{encode, Header, EncodingKey, Validation, decode, DecodingKey};
use serde::{Serialize, Deserialize};

use crate::ingest::AppError;

use super::cookie::COOKIE_NAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub aud: String,
    pub sub: Option<String>,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
}

pub struct ClaimsBuilder {
    pub iss: String,
    pub aud: String,
    pub sub: Option<String>,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
}

impl ClaimsBuilder {
    fn new() -> Self {
        Self {
            iss: "wuxia2kindle".to_owned(),
            aud: "wuxia2kindle".to_owned(),
            sub: None,
            exp: None,
            iat: None,
        }
    }

    fn sub(mut self, sub: String) -> Self {
        self.sub = Some(sub);
        self
    }

    fn exp(mut self, exp: u64) -> Self {
        self.exp = Some(exp);
        self
    }

    fn iat(mut self, iat: u64) -> Self {
        self.iat = Some(iat);
        self
    }

    fn build(self) -> Claims {
        Claims {
            iss: self.iss,
            aud: self.aud,
            sub: self.sub,
            exp: self.exp,
            iat: self.iat,
        }
    }
}

pub struct JWT {
    pub name: String,
    pub payload: String,
    pub domain: String,
    pub max_age: u64,
    pub path: String,
}

impl JWT {
    pub fn new(user: String) -> Self {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 15 minutes
        let exp = now + 15 * 60;

        let claims = ClaimsBuilder::new().sub(user).exp(exp).iat(now).build();

        let signed_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .expect("Failed to encode token");

        Self {
            name: COOKIE_NAME.to_owned(),
            payload: signed_token,
            domain,
            max_age: 15 * 60,
            path: "/".to_owned(),
        }
    }

    pub fn verify(token: String) -> Result<Claims, AppError> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&["wuxia2kindle"]);
        validation.set_issuer(&["wuxia2kindle"]);

        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;

        Ok(decoded.claims)
    }
}

impl Display for JWT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cookie = format!(
            "{}={}; Domain={}; Max-Age={}; Path={}; Secure; HttpOnly; SameSite=Strict",
            self.name, self.payload, self.domain, self.max_age, self.path
        );

        write!(f, "{}", cookie)
    }
}
