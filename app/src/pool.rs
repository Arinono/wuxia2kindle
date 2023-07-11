use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn mk_pool(url: String) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();

    pool
}

