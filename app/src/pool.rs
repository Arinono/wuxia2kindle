use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};

pub async fn mk_pool(url: String) -> PgPool {
    let opts: PgConnectOptions = url.parse().unwrap();
    let f_opts = opts.clone().log_statements(LevelFilter::Trace);

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(f_opts)
        .await
        .unwrap()
}
