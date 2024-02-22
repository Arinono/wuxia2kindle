mod pool;
mod server;
mod worker;

use std::fmt::Write;

use clap::{Parser, Subcommand};
use sha2::{Digest, Sha384};

#[derive(Debug, Parser)]
#[command(name = "wuxia2kindle")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = false)]
    Server,
    Worker,
    Checksum {
        #[clap(short, long)]
        file: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Server => {
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            std::env::var("DOMAIN").expect("DOMAIN must be set");
            std::env::var("SALT").expect("SALT must be set");

            let env_port = std::env::var("PORT").unwrap_or("3000".to_owned());
            let env_db_url = std::env::var("DATABASE_URL")
                .unwrap_or("postgres://localhost:5432/wuxia2kindle".to_owned());

            let port = env_port.parse::<u16>().expect("PORT must be a number");

            server::start(port, env_db_url);
        }
        Commands::Worker => {
            let env_db_url = std::env::var("DATABASE_URL")
                .unwrap_or("postgres://localhost:5432/wuxia2kindle".to_owned());
            let webhook_url = std::env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOKmust be set");

            worker::start(env_db_url, webhook_url);
        }
        Commands::Checksum { file } => {
            let file = std::fs::read(file).expect("Failed to read file");
            let checksum = Vec::from(Sha384::digest(file).as_slice());

            println!("{}", short_checksum(&checksum));
        }
    }
}

fn short_checksum(checksum: &[u8]) -> String {
    let mut s = String::with_capacity(checksum.len() * 2);
    for b in checksum {
        write!(&mut s, "{b:02x?}").expect("should not fail to write to str");
    }
    s
}
