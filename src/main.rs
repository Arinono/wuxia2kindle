mod env;
mod pool;
mod server;

use clap::Parser;

use env::Environment;

#[derive(Debug, Parser)]
#[command(name = "wuxia2kindle")]
struct Args {}

fn main() {
    let _args = Args::parse();
    let env = Environment::new();

    if let Ok(sentry_dsn) = std::env::var("SENTRY_DSN") {
        let _guard = sentry::init((sentry_dsn, sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        }));
    }

    server::start(env);
}
