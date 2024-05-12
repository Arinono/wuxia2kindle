pub mod book;
pub mod chapter;
pub mod epub;
pub mod export;
pub mod user;

#[cfg(feature = "sqlx")]
pub mod repository {
    use sqlx::PgPool;

    #[cfg(feature = "sqlx")]
    pub enum RepositoryError {
        NotFound,
        AlreadyExists,
        DatabaseError(sqlx::Error),
    }

    #[allow(async_fn_in_trait)] // i have to look into this warning
    pub trait Repository
    where
        Self: Sized,
    {
        async fn get_all(
            pool: &PgPool,
            offset: Option<i32>,
            rows_per_page: Option<i64>,
        ) -> Result<Vec<Self>, RepositoryError>;
        async fn get_by_id(pool: &PgPool, id: i32) -> Result<Self, RepositoryError>;
        async fn create(&self, pool: &PgPool) -> Result<Self, RepositoryError>;
        async fn update(&self, pool: &PgPool) -> Result<Self, RepositoryError>;
        async fn delete(&self, pool: &PgPool) -> Result<Self, RepositoryError>;
        async fn delete_by_id(pool: &PgPool, id: i32) -> Result<Self, RepositoryError>;
    }
}
