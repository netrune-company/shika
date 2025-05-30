use std::process::exit;

use clap::Parser;
use dotenvy::dotenv;
use shika::commands;
use shika_workspace::Workspace;

#[derive(Parser, Clone, Debug)]
struct Config {
    #[clap(subcommand)]
    command: Command,

    #[arg(long, env)]
    pub db_name: String,

    #[arg(long, env)]
    pub db_host: String,

    #[arg(long, env)]
    pub db_port: String,

    #[arg(long, env)]
    pub db_user: String,

    #[arg(long, env)]
    pub db_pass: String,
}

impl Config {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_pass, self.db_host, self.db_port, self.db_name
        )
    }
}

#[derive(clap::Subcommand, Clone, Debug)]
enum Command {
    Generate {
        template: Option<String>,
        #[clap(long, short, default_value = "false")]
        pull: bool,
    },
}

fn main() {
    dotenv().ok();

    let config = Config::parse();

    match config.command {
        Command::Generate { ref template, pull } => {
            if pull {
                match commands::pull::command(config.database_url()) {
                    Ok(_) => println!("Successfully pulled the latest changes from the database."),
                    Err(e) => {
                        eprintln!("Failed to pull database: {e}");
                        exit(1);
                    }
                }
            }

            match commands::generate::command(template.clone()) {
                Ok(_) => println!("Successfully generated the project from the template."),
                Err(e) => {
                    eprintln!("Failed to generate project: {e}");
                    exit(1);
                }
            }
        }
    }
}
