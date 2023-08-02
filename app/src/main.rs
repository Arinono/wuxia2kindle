mod epub;
mod ingest;
mod models;
mod pool;
mod worker;

use clap::{Parser, Subcommand};
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

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
        #[arg(long)]
        smtp_server: Option<String>,
        #[arg(long)]
        smtp_port: Option<u16>,
        #[arg(long)]
        smtp_user: Option<String>,
        #[arg(long)]
        smtp_password: Option<String>,
        #[arg(long)]
        send_to: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Ingest { port, database_url } => {
            let env_port = std::env::var("PORT");
            let env_db_url = std::env::var("DATABASE_URL");

            let f_port = match env_port {
                Ok(p) => p.parse::<u16>().unwrap(),
                Err(_) => port.unwrap_or(3000u16),
            };

            let db_url = match env_db_url {
                Ok(u) => u,
                Err(_) => match database_url {
                    Some(u) => u,
                    None => "postgres://localhost:5432/wuxia2kindle".to_owned(),
                },
            };

            ingest::start(f_port, db_url);
        }
        Commands::Worker {
            database_url,
            smtp_server,
            smtp_user,
            smtp_password,
            send_to,
            smtp_port,
        } => {
            let env_db_url = std::env::var("DATABASE_URL");
            let env_smtp_server = std::env::var("SMTP_SERVER");
            let env_smtp_port = std::env::var("SMTP_PORT");
            let env_smtp_user = std::env::var("SMTP_USER");
            let env_smtp_password = std::env::var("SMTP_PASSWORD");
            let env_send_to = std::env::var("SEND_TO");

            let db_url = match env_db_url {
                Ok(u) => u,
                Err(_) => match database_url {
                    Some(u) => u,
                    None => "postgres://localhost:5432/wuxia2kindle".to_owned(),
                },
            };

            let server = match env_smtp_server {
                Ok(s) => s,
                Err(_) => match smtp_server {
                    Some(s) => s,
                    None => panic!("smtp_server must be set"),
                },
            };

            let port = match env_smtp_port {
                Ok(p) => p.parse::<u16>().unwrap_or(25u16),
                Err(_) => smtp_port.unwrap_or(25u16),
            };

            let user = match env_smtp_user {
                Ok(u) => u,
                Err(_) => match smtp_user {
                    Some(u) => u,
                    None => panic!("smtp_user must be set"),
                },
            };

            let password = match env_smtp_password {
                Ok(p) => p,
                Err(_) => match smtp_password {
                    Some(p) => p,
                    None => panic!("smtp_password must be set"),
                },
            };

            let send_to = match env_send_to {
                Ok(s) => s,
                Err(_) => match send_to {
                    Some(s) => s,
                    None => panic!("send_to must be set"),
                },
            };

            let credentials = Credentials::new(user.clone(), password);
            let mailer = SmtpTransport::starttls_relay(server.as_ref())
                .unwrap()
                .port(port)
                .credentials(credentials)
                .build();

            mailer.test_connection().unwrap();

            worker::start(db_url, mailer, user, send_to);
        }
    }
}
