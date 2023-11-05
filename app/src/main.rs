mod pool;
mod server;
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
    Server,
    Worker,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Server => {
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            std::env::var("DOMAIN").expect("DOMAIN must be set");

            let env_port = std::env::var("PORT").unwrap_or("3000".to_owned());
            let env_db_url = std::env::var("DATABASE_URL")
                .unwrap_or("postgres://localhost:5432/wuxia2kindle".to_owned());

            let port = env_port.parse::<u16>().expect("PORT must be a number");

            server::start(port, env_db_url);
        }
        Commands::Worker => {
            let env_db_url = std::env::var("DATABASE_URL")
                .unwrap_or("postgres://localhost:5432/wuxia2kindle".to_owned());
            let env_smtp_server = std::env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
            let env_smtp_port = std::env::var("SMTP_PORT").expect("SMTP_PORT must be set");
            let env_smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
            let env_smtp_password =
                std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
            let env_send_to = std::env::var("SEND_TO").expect("SEND_TO must be set");

            let port = env_smtp_port
                .parse::<u16>()
                .expect("SMTP_PORT must be a number");

            let credentials = Credentials::new(env_smtp_user.clone(), env_smtp_password);
            let mailer = SmtpTransport::starttls_relay(&env_smtp_server)
                .unwrap()
                .port(port)
                .credentials(credentials)
                .build();

            mailer.test_connection().unwrap();

            worker::start(env_db_url, mailer, env_smtp_user, env_send_to);
        }
    }
}
