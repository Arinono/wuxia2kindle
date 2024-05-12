#[cfg(feature = "sqlx")]
use sqlx::PgPool;

#[cfg(feature = "sqlx")]
use crate::repository::{Repository, RepositoryError};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub discord_id: Option<String>,
    pub username: String,
    pub avatar: Option<String>,
    pub token: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: 0,
            discord_id: None,
            username: "".to_string(),
            avatar: None,
            token: None,
        }
    }

    pub fn from(username: String, discord_id: Option<String>, avatar: Option<String>) -> Self {
        Self {
            id: 0,
            discord_id,
            username,
            avatar,
            token: None,
        }
    }

    #[cfg(feature = "sqlx")]
    pub async fn get_by_discord_id(
        pool: &PgPool,
        discord_id: String,
    ) -> Result<Self, RepositoryError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE discord_id = $1 LIMIT 1",
            discord_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))?;

        match user {
            Some(user) => Ok(user),
            None => Err(RepositoryError::NotFound),
        }
    }
}

#[cfg(feature = "sqlx")]
impl Repository for User {
    async fn get_by_id(pool: &PgPool, id: i32) -> Result<Self, RepositoryError> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1", id)
            .fetch_optional(pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e))?;

        match user {
            Some(user) => Ok(user),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn get_all(
        pool: &PgPool,
        offset: Option<i32>,
        rows_per_page: Option<i64>,
    ) -> Result<Vec<Self>, RepositoryError> {
        let limit = rows_per_page.unwrap_or(50i64);
        let offset = match offset {
            Some(offset) => offset - 1,
            None => 0,
        };

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE id > $1
            ORDER BY id LIMIT $2
            "#,
            offset,
            limit,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))?;

        Ok(users)
    }

    async fn create(&self, pool: &PgPool) -> Result<Self, RepositoryError> {
        if let Ok(_) = Self::get_by_id(pool, self.id).await {
            return Err(RepositoryError::AlreadyExists);
        }

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (discord_id, username, avatar, token)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            self.discord_id,
            self.username,
            self.avatar,
            self.token,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))?;

        Ok(user)
    }

    async fn update(&self, pool: &PgPool) -> Result<Self, RepositoryError> {
        if let Err(err) = Self::get_by_id(pool, self.id).await {
            return Err(err);
        }

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET discord_id = $1, username = $2, avatar = $3, token = $4
            WHERE id = $5
            RETURNING *
            "#,
            self.discord_id,
            self.username,
            self.avatar,
            self.token,
            self.id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))?;

        Ok(user)
    }

    async fn delete(&self, pool: &PgPool) -> Result<Self, RepositoryError> {
        if let Err(err) = Self::get_by_id(pool, self.id).await {
            return Err(err);
        }

        let user = sqlx::query_as!(
            User,
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING *
            "#,
            self.id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))?;

        Ok(user)
    }

    async fn delete_by_id(pool: &PgPool, id: i32) -> Result<Self, RepositoryError> {
        if let Err(err) = Self::get_by_id(pool, id).await {
            return Err(err);
        }

        let user = sqlx::query_as!(
            User,
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e))?;

        Ok(user)
    }
}
