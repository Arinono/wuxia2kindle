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

    server::start(env);
}
