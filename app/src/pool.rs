use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn mk_pool(url: String) -> PgPool {
    

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap()
}

