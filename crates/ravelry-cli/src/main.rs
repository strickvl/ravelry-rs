//! Ravelry CLI - Command-line interface for the Ravelry API.

use clap::{Parser, Subcommand};
use ravelry::{
    api::patterns::PatternSearchParams,
    auth::BasicAuth,
    RavelryClient, RavelryError,
};

#[derive(Parser)]
#[command(name = "ravelry")]
#[command(author, version, about = "Command-line interface for the Ravelry API")]
struct Cli {
    /// Ravelry API access key (or set RAVELRY_ACCESS_KEY)
    #[arg(long, env = "RAVELRY_ACCESS_KEY", global = true)]
    access_key: Option<String>,

    /// Ravelry API personal key (or set RAVELRY_PERSONAL_KEY)
    #[arg(long, env = "RAVELRY_PERSONAL_KEY", global = true)]
    personal_key: Option<String>,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    /// Output as pretty-printed JSON
    #[arg(long, global = true)]
    json_pretty: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the current authenticated user
    Whoami,

    /// Pattern commands
    #[command(subcommand)]
    Patterns(PatternCommands),
}

#[derive(Subcommand)]
enum PatternCommands {
    /// Search for patterns
    Search {
        /// Search query
        #[arg(short, long)]
        query: Option<String>,

        /// Filter by craft (knitting, crochet)
        #[arg(long)]
        craft: Option<String>,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "10")]
        page_size: u32,
    },
}

impl Cli {
    /// Build a Ravelry client from CLI arguments.
    fn build_client(&self) -> Result<RavelryClient, CliError> {
        let access_key = self.access_key.clone().ok_or(CliError::MissingCredentials(
            "Access key required. Set --access-key or RAVELRY_ACCESS_KEY",
        ))?;
        let personal_key = self.personal_key.clone().ok_or(CliError::MissingCredentials(
            "Personal key required. Set --personal-key or RAVELRY_PERSONAL_KEY",
        ))?;

        let auth = BasicAuth::new(access_key, personal_key);
        let client = RavelryClient::builder(auth).build()?;
        Ok(client)
    }

    /// Check if JSON output is requested.
    fn json_output(&self) -> bool {
        self.json || self.json_pretty
    }

    /// Print a value as JSON (respecting --json-pretty flag).
    fn print_json<T: serde::Serialize>(&self, value: &T) -> Result<(), CliError> {
        let output = if self.json_pretty {
            serde_json::to_string_pretty(value)?
        } else {
            serde_json::to_string(value)?
        };
        println!("{output}");
        Ok(())
    }
}

#[derive(Debug)]
enum CliError {
    MissingCredentials(&'static str),
    Api(RavelryError),
    Json(serde_json::Error),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingCredentials(msg) => write!(f, "{msg}"),
            CliError::Api(e) => write!(f, "API error: {e}"),
            CliError::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<RavelryError> for CliError {
    fn from(e: RavelryError) -> Self {
        CliError::Api(e)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        CliError::Json(e)
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), CliError> {
    match &cli.command {
        Commands::Whoami => {
            let client = cli.build_client()?;
            let response = client.root().current_user().await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let user = &response.user;
                println!("Logged in as: {}", user.username);
                if let Some(name) = &user.name {
                    println!("Name: {name}");
                }
                println!("ID: {}", user.id);
            }
        }

        Commands::Patterns(PatternCommands::Search {
            query,
            craft,
            page,
            page_size,
        }) => {
            let client = cli.build_client()?;

            let mut params = PatternSearchParams::new()
                .page(*page)
                .page_size(*page_size);

            if let Some(q) = query {
                params = params.query(q);
            }
            if let Some(c) = craft {
                params = params.craft(c);
            }

            let response = client.patterns().search(&params).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!(
                    "Found {} patterns (page {}/{})",
                    response.paginator.results,
                    response.paginator.page,
                    response.paginator.page_count
                );
                println!();

                for pattern in &response.patterns {
                    let designer = pattern.designer_name.as_deref().unwrap_or("Unknown");
                    let free = if pattern.free.unwrap_or(false) { " [FREE]" } else { "" };
                    println!("  {} - {} by {}{}", pattern.id, pattern.name, designer, free);
                }
            }
        }
    }

    Ok(())
}
