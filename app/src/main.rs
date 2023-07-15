mod models;
mod epub;
mod ingest;
mod worker;
mod pool;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "wuxia2kindle")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = false)]
    Ingest {
        #[arg(short, long)]
        port: Option<u16>,
        #[arg(long)]
        database_url: Option<String>,
    },
    Worker {
        #[arg(long)]
        database_url: Option<String>,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Ingest { port, database_url } => {
            let env_port = std::env::var("PORT");
            let env_db_url = std::env::var("DATABASE_URL");

            let f_port = match env_port {
                Ok(p) => p.parse::<u16>().unwrap(),
                Err(_) => match port {
                    Some(p) => p,
                    None => 3000u16,
                }
            };

            let db_url = match env_db_url {
                Ok(u) => u,
                Err(_) => match database_url {
                    Some(u) => u,
                    None => "postgres://localhost:5432/wuxia2kindle".to_owned(),
                }
            };

            ingest::start(f_port, db_url);
        }
        Commands::Worker { database_url } => {
            let env_db_url = std::env::var("DATABASE_URL");

            let db_url = match env_db_url {
                Ok(u) => u,
                Err(_) => match database_url {
                    Some(u) => u,
                    None => "postgres://localhost:5432/wuxia2kindle".to_owned(),
                }
            };

            worker::start(db_url);
        }
    }
}
