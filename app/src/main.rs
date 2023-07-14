mod models;
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
    #[command(arg_required_else_help = true)]
    Ingest {
        #[arg(short, long)]
        port: u16,
        #[arg(long)]
        database_url: String,
    },
    Worker {
        #[arg(long)]
        database_url: String,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Ingest { port, database_url } => {
            ingest::start(port, database_url);
        }
        Commands::Worker { database_url } => {
            worker::start(database_url);
        }
    }
}
