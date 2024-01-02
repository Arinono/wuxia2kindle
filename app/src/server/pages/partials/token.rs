use askama::Template;
use axum::extract::State;
use bcrypt::{hash, DEFAULT_COST};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;

use crate::server::{auth::user::User, Error};

#[derive(Template)]
#[template(path = "partials/token.html")]
pub struct GetTokenTemplate {
    token: String,
}

pub async fn get_token(user: User, State(pool): State<PgPool>) -> Result<GetTokenTemplate, Error> {
    let thread_rng = rand::thread_rng();
    let mut token: String = thread_rng
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    token = token.to_uppercase();

    let hashed = hash(&token, DEFAULT_COST)?;

    let result = sqlx::query!("UPDATE users SET token = $1 WHERE id = $2", hashed, user.id)
        .execute(&pool)
        .await;

    match result {
        Err(e) => {
            println!("Error updating token: {}", e);
            return Err(Error::AppError(e.into()));
        },
        Ok(_) => {
            Ok(GetTokenTemplate {
                token,
            })
        },
    }
}
