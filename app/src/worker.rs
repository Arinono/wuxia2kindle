use std::time::Duration;

use sqlx::{pool::PoolConnection, Postgres};
use tokio::time::interval;

use crate::pool;

#[tokio::main]
pub async fn start(database_url: String) {
    let pool = pool::mk_pool(database_url.clone()).await;
    let mut interval = interval(Duration::from_secs(60));

    loop {
        let connection = pool.acquire().await.unwrap();
        tokio::spawn(async move {
            export(connection).await.close().await.unwrap();
        });
        interval.tick().await;
    }
}

async fn export(connection: PoolConnection<Postgres>) -> PoolConnection<Postgres> {
    let exports: Vec<Export> = {
    };

    connection
}
