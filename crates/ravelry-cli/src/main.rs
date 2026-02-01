//! Ravelry CLI - Command-line interface for the Ravelry API.

mod config;

use clap::{Parser, Subcommand};
use config::{Config, ConfigError, Profile};
use ravelry::{
    api::{
        bundles::BundlesListParams,
        favorites::FavoritesListParams,
        friends::FriendsActivityParams,
        messages::{MessageFolder, MessagesListParams},
        patterns::{PatternProjectsParams, PatternSearchParams},
        projects::ProjectsListParams,
        stash::StashListParams,
        yarns::YarnSearchParams,
    },
    auth::{BasicAuth, OAuth2Auth},
    pagination::collect_all_pages,
    types::{BookmarkPost, BundlePost, MessagePost, ProjectPost, StashPost, UploadFile},
    RavelryClient, RavelryError, RavelryOAuth2Client,
};
use std::time::Duration;

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

    /// Use a specific profile
    #[arg(long, short = 'p', global = true)]
    profile: Option<String>,

    /// Output as JSON
    #[arg(long, global = true, conflicts_with = "json_pretty")]
    json: bool,

    /// Output as pretty-printed JSON
    #[arg(long, global = true, conflicts_with = "json")]
    json_pretty: bool,

    /// Enable debug mode (adds debug info to API responses)
    #[arg(long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the current authenticated user
    Whoami,

    /// Authentication and profile management
    #[command(subcommand)]
    Auth(AuthCommands),

    /// Pattern commands
    #[command(subcommand)]
    Patterns(PatternCommands),

    /// Yarn commands
    #[command(subcommand)]
    Yarns(YarnCommands),

    /// Project commands
    #[command(subcommand)]
    Projects(ProjectCommands),

    /// Stash commands
    #[command(subcommand)]
    Stash(StashCommands),

    /// Message commands
    #[command(subcommand)]
    Messages(MessageCommands),

    /// Upload commands
    #[command(subcommand)]
    Upload(UploadCommands),

    /// Favorites commands
    #[command(subcommand)]
    Favorites(FavoriteCommands),

    /// Bundle commands
    #[command(subcommand)]
    Bundles(BundleCommands),

    /// Friend commands
    #[command(subcommand)]
    Friends(FriendCommands),
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login with OAuth2
    Login {
        /// OAuth2 client ID
        #[arg(long, env = "RAVELRY_CLIENT_ID")]
        client_id: String,

        /// OAuth2 client secret
        #[arg(long, env = "RAVELRY_CLIENT_SECRET")]
        client_secret: String,

        /// Profile name to save as
        #[arg(long, default_value = "default")]
        profile_name: String,

        /// OAuth2 scopes (space-separated)
        #[arg(long, default_value = "offline")]
        scopes: String,
    },

    /// Refresh OAuth2 tokens for a profile
    Refresh {
        /// Profile name to refresh (uses current profile if not specified)
        #[arg(long)]
        profile_name: Option<String>,
    },

    /// Set up basic auth credentials
    Basic {
        /// Access key
        #[arg(long, env = "RAVELRY_ACCESS_KEY")]
        access_key: String,

        /// Personal key
        #[arg(long, env = "RAVELRY_PERSONAL_KEY")]
        personal_key: String,

        /// Profile name to save as
        #[arg(long, default_value = "default")]
        profile_name: String,
    },

    /// List available profiles
    Profiles,

    /// Switch to a different profile
    Use {
        /// Profile name to switch to
        name: String,
    },

    /// Delete a profile
    Delete {
        /// Profile name to delete
        name: String,
    },

    /// Show the current authenticated user (alias for top-level whoami)
    Whoami,
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

        /// Fetch all pages
        #[arg(long)]
        all: bool,
    },

    /// Show pattern details
    Show {
        /// Pattern ID
        id: u64,
    },

    /// List projects made from a pattern
    Projects {
        /// Pattern ID
        id: u64,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "10")]
        page_size: u32,

        /// Fetch all pages
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand)]
enum YarnCommands {
    /// Search for yarns
    Search {
        /// Search query
        #[arg(short, long)]
        query: Option<String>,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "10")]
        page_size: u32,

        /// Fetch all pages
        #[arg(long)]
        all: bool,
    },

    /// Show yarn details
    Show {
        /// Yarn ID
        id: u64,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// List projects for a user
    List {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "10")]
        page_size: u32,

        /// Fetch all pages
        #[arg(long)]
        all: bool,
    },

    /// Show project details
    Show {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Project ID or permalink
        id: String,
    },

    /// Create a new project
    Create {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Project name
        #[arg(long)]
        name: String,

        /// Pattern ID to link to
        #[arg(long)]
        pattern_id: Option<u64>,

        /// Status ID (1=in progress, 2=finished, etc.)
        #[arg(long)]
        status_id: Option<u64>,

        /// Craft ID (1=knitting, 2=crochet, etc.)
        #[arg(long)]
        craft_id: Option<u64>,
    },

    /// Update an existing project
    Update {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Project ID
        id: u64,

        /// New project name
        #[arg(long)]
        name: Option<String>,

        /// New status ID
        #[arg(long)]
        status_id: Option<u64>,

        /// New progress percentage (0-100)
        #[arg(long)]
        progress: Option<u32>,

        /// New notes
        #[arg(long)]
        notes: Option<String>,
    },
}

#[derive(Subcommand)]
enum StashCommands {
    /// List stash for a user
    List {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "10")]
        page_size: u32,

        /// Fetch all pages
        #[arg(long)]
        all: bool,
    },

    /// Show stash entry details
    Show {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Stash ID or permalink
        id: String,
    },

    /// Create a new stash entry
    Create {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Yarn ID to add to stash
        #[arg(long)]
        yarn_id: u64,

        /// Colorway name
        #[arg(long)]
        colorway: Option<String>,

        /// Number of skeins
        #[arg(long)]
        skeins: Option<f64>,

        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },

    /// Update an existing stash entry
    Update {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Stash ID
        id: u64,

        /// Colorway name
        #[arg(long)]
        colorway: Option<String>,

        /// Number of skeins
        #[arg(long)]
        skeins: Option<f64>,

        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },
}

#[derive(Subcommand)]
enum MessageCommands {
    /// List messages
    List {
        /// Folder (inbox, sent, archived)
        #[arg(long, default_value = "inbox")]
        folder: String,

        /// Only show unread messages
        #[arg(long)]
        unread: bool,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "20")]
        page_size: u32,
    },

    /// Read a message
    Read {
        /// Message ID
        id: u64,
    },

    /// Mark a message as read
    MarkRead {
        /// Message ID
        id: u64,
    },

    /// Mark a message as unread
    MarkUnread {
        /// Message ID
        id: u64,
    },

    /// Archive a message
    Archive {
        /// Message ID
        id: u64,
    },

    /// Delete a message
    Delete {
        /// Message ID
        id: u64,
    },

    /// Send a new message
    Send {
        /// Recipient username
        #[arg(long)]
        to: String,

        /// Message subject
        #[arg(long)]
        subject: String,

        /// Message content
        #[arg(long)]
        content: String,
    },

    /// Reply to a message
    Reply {
        /// Message ID to reply to
        id: u64,

        /// Reply content
        #[arg(long)]
        content: String,
    },

    /// Unarchive a message
    Unarchive {
        /// Message ID
        id: u64,
    },
}

#[derive(Subcommand)]
enum UploadCommands {
    /// Upload one or more images
    Image {
        /// File paths to upload
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
enum FavoriteCommands {
    /// List favorites for a user
    List {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Filter by type (pattern, yarn, etc.)
        #[arg(long)]
        type_filter: Option<String>,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "10")]
        page_size: u32,
    },

    /// Show a specific favorite
    Show {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Favorite ID
        id: u64,
    },

    /// Create a new favorite
    Create {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Type of item to favorite (pattern, yarn, etc.)
        #[arg(long)]
        item_type: String,

        /// ID of the item to favorite
        #[arg(long)]
        item_id: u64,

        /// Comment on the favorite
        #[arg(long)]
        comment: Option<String>,
    },

    /// Delete a favorite
    Delete {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Favorite ID
        id: u64,
    },
}

#[derive(Subcommand)]
enum BundleCommands {
    /// List bundles for a user
    List {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "10")]
        page_size: u32,
    },

    /// Show a specific bundle
    Show {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Bundle ID
        id: u64,
    },

    /// Create a new bundle
    Create {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Bundle name
        #[arg(long)]
        name: String,

        /// Make bundle public
        #[arg(long)]
        public: bool,
    },

    /// Delete a bundle
    Delete {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Bundle ID
        id: u64,
    },
}

#[derive(Subcommand)]
enum FriendCommands {
    /// List friends for a user
    List {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,
    },

    /// Show friend activity feed
    Activity {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Page number
        #[arg(long, default_value = "1")]
        page: u32,

        /// Results per page
        #[arg(long, default_value = "20")]
        page_size: u32,
    },

    /// Add a friend (follow a user)
    Add {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// User ID of the person to follow
        friend_user_id: u64,
    },

    /// Remove a friend (unfollow)
    Remove {
        /// Username (uses current user if not specified)
        #[arg(long)]
        user: Option<String>,

        /// Friendship ID to remove
        friendship_id: u64,
    },
}

impl Cli {
    /// Build a Ravelry client from CLI arguments or config.
    async fn build_client(&self) -> Result<RavelryClient, CliError> {
        // First, try CLI args (backward compatible)
        if let (Some(access_key), Some(personal_key)) = (&self.access_key, &self.personal_key) {
            let auth = BasicAuth::new(access_key, personal_key);
            return Ok(RavelryClient::builder(auth).debug(self.debug).build()?);
        }

        // Otherwise, try config profiles
        let config = Config::load()?;

        // Determine which profile to use
        let profile_name = self
            .profile
            .as_deref()
            .or(config.current_profile.as_deref());

        let profile = profile_name
            .and_then(|name| config.get_profile(name))
            .ok_or(CliError::MissingCredentials(
                "No credentials. Use --access-key/--personal-key, --profile, or run 'ravelry auth basic'",
            ))?;

        match profile {
            Profile::Basic {
                access_key,
                personal_key,
            } => {
                let auth = BasicAuth::new(access_key, personal_key);
                Ok(RavelryClient::builder(auth).debug(self.debug).build()?)
            }
            Profile::OAuth2 {
                client_id,
                client_secret,
                token,
            } => {
                // Check if token needs refresh
                let mut token = token.clone();
                if token.is_expired(Duration::from_secs(300)) {
                    if let Some(refresh_token) = &token.refresh_token {
                        eprintln!("Token expired, refreshing...");
                        let oauth_client = RavelryOAuth2Client::new(
                            client_id,
                            client_secret,
                            "https://localhost:8080/callback",
                        )?;
                        token = oauth_client.refresh(refresh_token).await?;

                        // Save updated token
                        let mut config = Config::load()?;
                        if let Some(name) = profile_name {
                            config.set_profile(
                                name,
                                Profile::oauth2(client_id, client_secret, token.clone()),
                            );
                            config.save()?;
                        }
                    } else {
                        return Err(CliError::MissingCredentials(
                            "OAuth2 token expired and no refresh token available. Please re-login.",
                        ));
                    }
                }

                let auth = OAuth2Auth::new(&token.access_token);
                Ok(RavelryClient::builder(auth).debug(self.debug).build()?)
            }
        }
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
    Config(ConfigError),
    Io(std::io::Error),
    Other(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingCredentials(msg) => write!(f, "{msg}"),
            CliError::Api(e) => write!(f, "API error: {e}"),
            CliError::Json(e) => write!(f, "JSON error: {e}"),
            CliError::Config(e) => write!(f, "Config error: {e}"),
            CliError::Io(e) => write!(f, "IO error: {e}"),
            CliError::Other(msg) => write!(f, "{msg}"),
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

impl From<ConfigError> for CliError {
    fn from(e: ConfigError) -> Self {
        CliError::Config(e)
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
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
            let client = cli.build_client().await?;
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

        Commands::Auth(auth_cmd) => run_auth_command(&cli, auth_cmd).await?,

        Commands::Patterns(pattern_cmd) => run_pattern_command(&cli, pattern_cmd).await?,

        Commands::Yarns(yarn_cmd) => run_yarn_command(&cli, yarn_cmd).await?,
        Commands::Projects(project_cmd) => run_project_command(&cli, project_cmd).await?,
        Commands::Stash(stash_cmd) => run_stash_command(&cli, stash_cmd).await?,
        Commands::Messages(message_cmd) => run_message_command(&cli, message_cmd).await?,
        Commands::Upload(upload_cmd) => run_upload_command(&cli, upload_cmd).await?,
        Commands::Favorites(fav_cmd) => run_favorites_command(&cli, fav_cmd).await?,
        Commands::Bundles(bundle_cmd) => run_bundles_command(&cli, bundle_cmd).await?,
        Commands::Friends(friend_cmd) => run_friends_command(&cli, friend_cmd).await?,
    }

    Ok(())
}

async fn run_auth_command(cli: &Cli, cmd: &AuthCommands) -> Result<(), CliError> {
    match cmd {
        AuthCommands::Login {
            client_id,
            client_secret,
            profile_name,
            scopes,
        } => {
            // Create OAuth2 client
            let oauth_client = RavelryOAuth2Client::new(
                client_id,
                client_secret,
                "https://localhost:8080/callback",
            )?;

            // Generate authorization URL
            let scope_list: Vec<String> = scopes.split_whitespace().map(String::from).collect();
            let (auth_url, _csrf_state) = oauth_client.authorize_url(scope_list);

            println!("Opening browser for authorization...");
            println!("If browser doesn't open, visit: {auth_url}");

            // Open browser
            if let Err(e) = open::that(auth_url.as_str()) {
                eprintln!("Failed to open browser: {e}");
            }

            // Start callback server
            println!("\nWaiting for callback on https://localhost:8080/callback ...");

            let code = wait_for_oauth_callback().await?;

            println!("Received authorization code, exchanging for tokens...");

            // Exchange code for tokens
            let token = oauth_client.exchange_code(&code).await?;

            // Save to config
            let mut config = Config::load()?;
            config.set_profile(
                profile_name,
                Profile::oauth2(client_id, client_secret, token),
            );
            config.set_current(profile_name);
            config.save()?;

            println!("Successfully logged in! Profile '{}' saved.", profile_name);

            if let Some(path) = Config::path() {
                println!("Config saved to: {}", path.display());
            }
        }

        AuthCommands::Basic {
            access_key,
            personal_key,
            profile_name,
        } => {
            let mut config = Config::load()?;
            config.set_profile(profile_name, Profile::basic(access_key, personal_key));
            config.set_current(profile_name);
            config.save()?;

            println!(
                "Basic auth profile '{}' saved as current profile.",
                profile_name
            );
        }

        AuthCommands::Profiles => {
            let config = Config::load()?;

            if config.profiles.is_empty() {
                println!("No profiles configured.");
                println!("Run 'ravelry auth basic' or 'ravelry auth login' to add one.");
            } else {
                println!("Available profiles:");
                for name in config.profile_names() {
                    let current = if config.current_profile.as_deref() == Some(name) {
                        " (current)"
                    } else {
                        ""
                    };
                    let profile = config.get_profile(name).unwrap();
                    let kind = match profile {
                        Profile::Basic { .. } => "basic",
                        Profile::OAuth2 { .. } => "oauth2",
                    };
                    println!("  {} [{}]{}", name, kind, current);
                }
            }
        }

        AuthCommands::Use { name } => {
            let mut config = Config::load()?;

            if config.get_profile(name).is_none() {
                eprintln!("Profile '{}' not found.", name);
                std::process::exit(1);
            }

            config.set_current(name);
            config.save()?;
            println!("Switched to profile '{}'.", name);
        }

        AuthCommands::Refresh { profile_name } => {
            let mut config = Config::load()?;

            // Determine which profile to refresh
            let name = profile_name
                .as_deref()
                .or(cli.profile.as_deref())
                .or(config.current_profile.as_deref())
                .ok_or(CliError::MissingCredentials(
                    "No profile specified. Use --profile-name or set a current profile.",
                ))?
                .to_string();

            let profile = config
                .get_profile(&name)
                .ok_or_else(|| CliError::Other(format!("Profile '{}' not found.", name)))?;

            match profile {
                Profile::OAuth2 {
                    client_id,
                    client_secret,
                    token,
                } => {
                    let refresh_token =
                        token
                            .refresh_token
                            .as_ref()
                            .ok_or(CliError::MissingCredentials(
                            "OAuth2 profile has no refresh token. Re-login with 'offline' scope.",
                        ))?;

                    let oauth_client = RavelryOAuth2Client::new(
                        client_id,
                        client_secret,
                        "https://localhost:8080/callback",
                    )?;

                    println!("Refreshing token for profile '{}'...", name);
                    let new_token = oauth_client.refresh(refresh_token).await?;

                    // Save updated token
                    config.set_profile(&name, Profile::oauth2(client_id, client_secret, new_token));
                    config.save()?;

                    println!("Token refreshed successfully.");
                }
                Profile::Basic { .. } => {
                    return Err(CliError::Other(
                        "Cannot refresh Basic auth profile. Refresh is only for OAuth2."
                            .to_string(),
                    ));
                }
            }
        }

        AuthCommands::Delete { name } => {
            let mut config = Config::load()?;

            if config.get_profile(name).is_none() {
                eprintln!("Profile '{}' not found.", name);
                std::process::exit(1);
            }

            config.delete_profile(name);
            config.save()?;
            println!("Deleted profile '{}'.", name);
        }

        AuthCommands::Whoami => {
            let client = cli.build_client().await?;
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
    }

    Ok(())
}

async fn run_pattern_command(cli: &Cli, cmd: &PatternCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        PatternCommands::Search {
            query,
            craft,
            page,
            page_size,
            all,
        } => {
            if *all {
                let all_patterns = collect_all_pages(*page_size, None, |page_params| {
                    let client = &client;
                    let query = query.clone();
                    let craft = craft.clone();
                    async move {
                        let mut params = PatternSearchParams {
                            page: page_params,
                            ..Default::default()
                        };
                        if let Some(q) = query {
                            params = params.query(q);
                        }
                        if let Some(c) = craft {
                            params = params.craft(c);
                        }
                        let resp = client.patterns().search(&params).await?;
                        Ok((resp.patterns, resp.paginator))
                    }
                })
                .await?;

                if cli.json_output() {
                    cli.print_json(&all_patterns)?;
                } else {
                    println!("Found {} patterns total", all_patterns.len());
                    for pattern in &all_patterns {
                        let designer = pattern.designer_name.as_deref().unwrap_or("Unknown");
                        let free = if pattern.free.unwrap_or(false) {
                            " [FREE]"
                        } else {
                            ""
                        };
                        println!(
                            "  {} - {} by {}{}",
                            pattern.id, pattern.name, designer, free
                        );
                    }
                }
            } else {
                let mut params = PatternSearchParams::new().page(*page).page_size(*page_size);

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
                        let free = if pattern.free.unwrap_or(false) {
                            " [FREE]"
                        } else {
                            ""
                        };
                        println!(
                            "  {} - {} by {}{}",
                            pattern.id, pattern.name, designer, free
                        );
                    }
                }
            }
        }

        PatternCommands::Show { id } => {
            let response = client.patterns().show(*id).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let pattern = &response.pattern;
                println!("Pattern: {}", pattern.name);
                println!("ID: {}", pattern.id);
                if let Some(designer) = &pattern.designer_name {
                    println!("Designer: {designer}");
                }
                if let Some(free) = pattern.free {
                    println!("Free: {}", if free { "Yes" } else { "No" });
                }
                if let Some(count) = pattern.projects_count {
                    println!("Projects: {count}");
                }
                if let Some(rating) = pattern.rating_average {
                    println!("Rating: {:.1}/5", rating);
                }
            }
        }

        PatternCommands::Projects {
            id,
            page,
            page_size,
            all,
        } => {
            if *all {
                let all_projects = collect_all_pages(*page_size, None, |page_params| {
                    let client = &client;
                    let id = *id;
                    async move {
                        let params = PatternProjectsParams {
                            page: page_params,
                            ..Default::default()
                        };
                        let resp = client.patterns().projects(id, &params).await?;
                        Ok((resp.projects, resp.paginator))
                    }
                })
                .await?;

                if cli.json_output() {
                    cli.print_json(&all_projects)?;
                } else {
                    println!("Found {} projects total", all_projects.len());
                    for project in &all_projects {
                        let status = project.status_name.as_deref().unwrap_or("Unknown");
                        println!("  {} - {} [{}]", project.id, project.name, status);
                    }
                }
            } else {
                let params = PatternProjectsParams::new()
                    .page(*page)
                    .page_size(*page_size);
                let response = client.patterns().projects(*id, &params).await?;

                if cli.json_output() {
                    cli.print_json(&response)?;
                } else {
                    println!(
                        "Projects for pattern {} (page {}/{})",
                        id, response.paginator.page, response.paginator.page_count
                    );
                    println!();

                    for project in &response.projects {
                        let status = project.status_name.as_deref().unwrap_or("Unknown");
                        println!("  {} - {} [{}]", project.id, project.name, status);
                    }
                }
            }
        }
    }

    Ok(())
}

async fn run_yarn_command(cli: &Cli, cmd: &YarnCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        YarnCommands::Search {
            query,
            page,
            page_size,
            all,
        } => {
            if *all {
                let all_yarns = collect_all_pages(*page_size, None, |page_params| {
                    let client = &client;
                    let query = query.clone();
                    async move {
                        let mut params = YarnSearchParams {
                            page: page_params,
                            ..Default::default()
                        };
                        if let Some(q) = query {
                            params = params.query(q);
                        }
                        let resp = client.yarns().search(&params).await?;
                        Ok((resp.yarns, resp.paginator))
                    }
                })
                .await?;

                if cli.json_output() {
                    cli.print_json(&all_yarns)?;
                } else {
                    println!("Found {} yarns total", all_yarns.len());
                    for yarn in &all_yarns {
                        let company = yarn.yarn_company_name.as_deref().unwrap_or("Unknown");
                        println!("  {} - {} by {}", yarn.id, yarn.name, company);
                    }
                }
            } else {
                let mut params = YarnSearchParams::new().page(*page).page_size(*page_size);

                if let Some(q) = query {
                    params = params.query(q);
                }

                let response = client.yarns().search(&params).await?;

                if cli.json_output() {
                    cli.print_json(&response)?;
                } else {
                    println!(
                        "Found {} yarns (page {}/{})",
                        response.paginator.results,
                        response.paginator.page,
                        response.paginator.page_count
                    );
                    println!();

                    for yarn in &response.yarns {
                        let company = yarn.yarn_company_name.as_deref().unwrap_or("Unknown");
                        println!("  {} - {} by {}", yarn.id, yarn.name, company);
                    }
                }
            }
        }

        YarnCommands::Show { id } => {
            let response = client.yarns().show(*id, &Default::default()).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let yarn = &response.yarn;
                println!("Yarn: {}", yarn.name);
                println!("ID: {}", yarn.id);
                if let Some(company) = &yarn.yarn_company_name {
                    println!("Company: {company}");
                }
                if let Some(weight) = &yarn.yarn_weight_name {
                    println!("Weight: {weight}");
                }
                if let Some(fiber) = &yarn.fiber_content {
                    println!("Fiber: {fiber}");
                }
            }
        }
    }

    Ok(())
}

/// Helper to resolve username from option or current user API call.
async fn resolve_username(
    client: &RavelryClient,
    user: &Option<String>,
) -> Result<String, CliError> {
    match user {
        Some(u) => Ok(u.clone()),
        None => {
            let response = client.root().current_user().await?;
            Ok(response.user.username)
        }
    }
}

async fn run_project_command(cli: &Cli, cmd: &ProjectCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        ProjectCommands::List {
            user,
            page,
            page_size,
            all,
        } => {
            let username = resolve_username(&client, user).await?;

            if *all {
                let all_projects = collect_all_pages(*page_size, None, |page_params| {
                    let client = &client;
                    let username = username.clone();
                    async move {
                        let params = ProjectsListParams {
                            page: page_params,
                            ..Default::default()
                        };
                        let resp = client.projects().list(&username, &params).await?;
                        Ok((resp.projects, resp.paginator))
                    }
                })
                .await?;

                if cli.json_output() {
                    cli.print_json(&all_projects)?;
                } else {
                    println!("Found {} projects total", all_projects.len());
                    for project in &all_projects {
                        let status = project.status_name.as_deref().unwrap_or("Unknown");
                        println!("  {} - {} [{}]", project.id, project.name, status);
                    }
                }
            } else {
                let params = ProjectsListParams::new().page(*page).page_size(*page_size);
                let response = client.projects().list(&username, &params).await?;

                if cli.json_output() {
                    cli.print_json(&response)?;
                } else {
                    println!(
                        "Found {} projects (page {}/{})",
                        response.paginator.results,
                        response.paginator.page,
                        response.paginator.page_count
                    );
                    println!();

                    for project in &response.projects {
                        let status = project.status_name.as_deref().unwrap_or("Unknown");
                        println!("  {} - {} [{}]", project.id, project.name, status);
                    }
                }
            }
        }

        ProjectCommands::Show { user, id } => {
            let username = resolve_username(&client, user).await?;
            let response = client
                .projects()
                .show(&username, id, &Default::default())
                .await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let project = &response.project;
                println!("Project: {}", project.name);
                println!("ID: {}", project.id);
                if let Some(pattern) = &project.pattern_name {
                    println!("Pattern: {pattern}");
                }
                if let Some(status) = &project.status_name {
                    println!("Status: {status}");
                }
                if let Some(progress) = project.progress {
                    println!("Progress: {}%", progress);
                }
            }
        }

        ProjectCommands::Create {
            user,
            name,
            pattern_id,
            status_id,
            craft_id,
        } => {
            let username = resolve_username(&client, user).await?;

            let mut post = ProjectPost::new().name(name);
            if let Some(pid) = pattern_id {
                post = post.pattern_id(*pid);
            }
            if let Some(sid) = status_id {
                post = post.status_id(*sid);
            }
            if let Some(cid) = craft_id {
                post.craft_id = Some(*cid);
            }

            let response = client.projects().create(&username, &post).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let project = &response.project;
                println!("Created project: {} (ID: {})", project.name, project.id);
                if let Some(status) = &project.status_name {
                    println!("Status: {status}");
                }
            }
        }

        ProjectCommands::Update {
            user,
            id,
            name,
            status_id,
            progress,
            notes,
        } => {
            let username = resolve_username(&client, user).await?;

            let mut post = ProjectPost::new();
            if let Some(n) = name {
                post = post.name(n);
            }
            if let Some(sid) = status_id {
                post = post.status_id(*sid);
            }
            if let Some(p) = progress {
                post = post.progress(*p);
            }
            if let Some(n) = notes {
                post.notes = Some(n.clone());
            }

            let response = client.projects().update(&username, *id, &post).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let project = &response.project;
                println!("Updated project: {} (ID: {})", project.name, project.id);
                if let Some(status) = &project.status_name {
                    println!("Status: {status}");
                }
                if let Some(prog) = project.progress {
                    println!("Progress: {}%", prog);
                }
            }
        }
    }

    Ok(())
}

async fn run_stash_command(cli: &Cli, cmd: &StashCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        StashCommands::List {
            user,
            page,
            page_size,
            all,
        } => {
            let username = resolve_username(&client, user).await?;

            if *all {
                // Try to use pagination if available
                let first_response = client
                    .stash()
                    .list(
                        &username,
                        &StashListParams::new().page(1).page_size(*page_size),
                    )
                    .await?;

                if let Some(paginator) = &first_response.paginator {
                    // Pagination available, fetch all pages
                    let mut all_stash = first_response.stash;
                    let total_pages = paginator.page_count;

                    for page_num in 2..=total_pages {
                        let params = StashListParams::new().page(page_num).page_size(*page_size);
                        let resp = client.stash().list(&username, &params).await?;
                        all_stash.extend(resp.stash);
                    }

                    if cli.json_output() {
                        cli.print_json(&all_stash)?;
                    } else {
                        println!("Found {} stash entries total", all_stash.len());
                        for entry in &all_stash {
                            let yarn = entry.yarn_name.as_deref().unwrap_or("Unknown yarn");
                            let color = entry.colorway_name.as_deref().unwrap_or("");
                            if color.is_empty() {
                                println!("  {} - {}", entry.id, yarn);
                            } else {
                                println!("  {} - {} ({})", entry.id, yarn, color);
                            }
                        }
                    }
                } else {
                    // No paginator, just return what we have
                    if cli.json_output() {
                        cli.print_json(&first_response.stash)?;
                    } else {
                        println!("Stash entries (pagination not available, showing first page):");
                        for entry in &first_response.stash {
                            let yarn = entry.yarn_name.as_deref().unwrap_or("Unknown yarn");
                            let color = entry.colorway_name.as_deref().unwrap_or("");
                            if color.is_empty() {
                                println!("  {} - {}", entry.id, yarn);
                            } else {
                                println!("  {} - {} ({})", entry.id, yarn, color);
                            }
                        }
                    }
                }
            } else {
                let params = StashListParams::new().page(*page).page_size(*page_size);
                let response = client.stash().list(&username, &params).await?;

                if cli.json_output() {
                    cli.print_json(&response)?;
                } else {
                    if let Some(paginator) = &response.paginator {
                        println!(
                            "Stash entries (page {}/{}):",
                            paginator.page, paginator.page_count
                        );
                    } else {
                        println!("Stash entries:");
                    }
                    for entry in &response.stash {
                        let yarn = entry.yarn_name.as_deref().unwrap_or("Unknown yarn");
                        let color = entry.colorway_name.as_deref().unwrap_or("");
                        if color.is_empty() {
                            println!("  {} - {}", entry.id, yarn);
                        } else {
                            println!("  {} - {} ({})", entry.id, yarn, color);
                        }
                    }
                }
            }
        }

        StashCommands::Show { user, id } => {
            let username = resolve_username(&client, user).await?;
            let response = client.stash().show(&username, id).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let stash = &response.stash;
                println!("Stash Entry: {}", stash.id);
                if let Some(yarn) = &stash.yarn_name {
                    println!("Yarn: {yarn}");
                }
                if let Some(color) = &stash.colorway_name {
                    println!("Colorway: {color}");
                }
                if let Some(lot) = &stash.dye_lot {
                    println!("Dye Lot: {lot}");
                }
                if let Some(skeins) = stash.skeins {
                    println!("Skeins: {skeins}");
                }
            }
        }

        StashCommands::Create {
            user,
            yarn_id,
            colorway,
            skeins,
            notes,
        } => {
            let username = resolve_username(&client, user).await?;

            let mut post = StashPost::new().yarn_id(*yarn_id);
            if let Some(c) = colorway {
                post = post.colorway_name(c);
            }
            if let Some(s) = skeins {
                post = post.skeins(*s);
            }
            if let Some(n) = notes {
                post.notes = Some(n.clone());
            }

            let response = client.stash().create(&username, &post).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let stash = &response.stash;
                println!("Created stash entry: {}", stash.id);
                if let Some(yarn) = &stash.yarn_name {
                    println!("Yarn: {yarn}");
                }
                if let Some(color) = &stash.colorway_name {
                    println!("Colorway: {color}");
                }
            }
        }

        StashCommands::Update {
            user,
            id,
            colorway,
            skeins,
            notes,
        } => {
            let username = resolve_username(&client, user).await?;

            let mut post = StashPost::new();
            if let Some(c) = colorway {
                post = post.colorway_name(c);
            }
            if let Some(s) = skeins {
                post = post.skeins(*s);
            }
            if let Some(n) = notes {
                post.notes = Some(n.clone());
            }

            let response = client.stash().update(&username, *id, &post).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let stash = &response.stash;
                println!("Updated stash entry: {}", stash.id);
                if let Some(yarn) = &stash.yarn_name {
                    println!("Yarn: {yarn}");
                }
                if let Some(color) = &stash.colorway_name {
                    println!("Colorway: {color}");
                }
                if let Some(sk) = stash.skeins {
                    println!("Skeins: {sk}");
                }
            }
        }
    }

    Ok(())
}

async fn run_message_command(cli: &Cli, cmd: &MessageCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        MessageCommands::List {
            folder,
            unread,
            page,
            page_size,
        } => {
            let folder_enum = match folder.as_str() {
                "inbox" => MessageFolder::Inbox,
                "sent" => MessageFolder::Sent,
                "archived" => MessageFolder::Archived,
                _ => {
                    eprintln!("Invalid folder: {}. Use inbox, sent, or archived.", folder);
                    std::process::exit(1);
                }
            };

            let params = MessagesListParams::new()
                .folder(folder_enum)
                .unread_only(*unread)
                .page(*page)
                .page_size(*page_size);

            let response = client.messages().list(&params).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!(
                    "Messages in {} (page {}/{})",
                    folder, response.paginator.page, response.paginator.page_count
                );
                println!();

                for message in &response.messages {
                    let read_marker = if message.read_message.unwrap_or(true) {
                        " "
                    } else {
                        "*"
                    };
                    let sender = message
                        .sender
                        .as_ref()
                        .map(|s| s.username.as_str())
                        .unwrap_or("Unknown");
                    println!(
                        "{}{} - {} (from {})",
                        read_marker, message.id, message.subject, sender
                    );
                }
            }
        }

        MessageCommands::Read { id } => {
            let response = client.messages().show(*id).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let message = &response.message;
                println!("Subject: {}", message.subject);
                if let Some(sender) = &message.sender {
                    println!("From: {}", sender.username);
                }
                if let Some(sent_at) = &message.sent_at {
                    println!("Sent: {sent_at}");
                }
                println!();
                if let Some(content) = &message.content {
                    println!("{content}");
                }
            }
        }

        MessageCommands::MarkRead { id } => {
            client.messages().mark_read(*id).await?;
            println!("Message {} marked as read.", id);
        }

        MessageCommands::MarkUnread { id } => {
            client.messages().mark_unread(*id).await?;
            println!("Message {} marked as unread.", id);
        }

        MessageCommands::Archive { id } => {
            client.messages().archive(*id).await?;
            println!("Message {} archived.", id);
        }

        MessageCommands::Delete { id } => {
            client.messages().delete(*id).await?;
            println!("Message {} deleted.", id);
        }

        MessageCommands::Send {
            to,
            subject,
            content,
        } => {
            let message = MessagePost::new()
                .recipient_username(to)
                .subject(subject)
                .content(content);

            let response = client.messages().create(&message).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Message sent! ID: {}", response.message.id);
            }
        }

        MessageCommands::Reply { id, content } => {
            let reply = MessagePost::new().content(content);
            let response = client.messages().reply(*id, &reply).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Reply sent! ID: {}", response.message.id);
            }
        }

        MessageCommands::Unarchive { id } => {
            client.messages().unarchive(*id).await?;
            println!("Message {} unarchived.", id);
        }
    }

    Ok(())
}

async fn run_upload_command(cli: &Cli, cmd: &UploadCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        UploadCommands::Image { files } => {
            // Request upload token
            let token_resp = client.upload().request_token().await?;
            println!("Got upload token, uploading {} file(s)...", files.len());

            // Read files
            let upload_files: Vec<UploadFile> = files
                .iter()
                .map(|path| {
                    let bytes = std::fs::read(path)?;
                    let filename = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("image")
                        .to_string();
                    Ok(UploadFile::new(filename, bytes))
                })
                .collect::<Result<Vec<_>, std::io::Error>>()?;

            // Upload
            let response = client
                .upload()
                .image(&token_resp.upload_token, upload_files)
                .await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Upload complete!");
                for upload in &response.uploads {
                    for (key, result) in upload {
                        println!("  {}: image_id = {}", key, result.image_id);
                    }
                }
            }
        }
    }

    Ok(())
}

async fn run_favorites_command(cli: &Cli, cmd: &FavoriteCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        FavoriteCommands::List {
            user,
            type_filter,
            page,
            page_size,
        } => {
            let username = resolve_username(&client, user).await?;

            let mut params = FavoritesListParams::new().page(*page).page_size(*page_size);
            if let Some(t) = type_filter {
                params = params.type_filter(t);
            }

            let response = client.favorites().list(&username, &params).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                if let Some(paginator) = &response.paginator {
                    println!(
                        "Favorites (page {}/{}):",
                        paginator.page, paginator.page_count
                    );
                } else {
                    println!("Favorites:");
                }
                for fav in &response.favorites {
                    let type_name = fav.type_name.as_deref().unwrap_or("unknown");
                    let comment = fav.comment.as_deref().unwrap_or("");
                    if comment.is_empty() {
                        println!("  {} - [{}] #{:?}", fav.id, type_name, fav.favorited_id);
                    } else {
                        println!(
                            "  {} - [{}] #{:?} - {}",
                            fav.id, type_name, fav.favorited_id, comment
                        );
                    }
                }
            }
        }

        FavoriteCommands::Show { user, id } => {
            let username = resolve_username(&client, user).await?;
            let response = client.favorites().show(&username, *id).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let fav = &response.favorite;
                println!("Favorite: {}", fav.id);
                if let Some(t) = &fav.type_name {
                    println!("Type: {t}");
                }
                if let Some(id) = fav.favorited_id {
                    println!("Item ID: {id}");
                }
                if let Some(c) = &fav.comment {
                    println!("Comment: {c}");
                }
            }
        }

        FavoriteCommands::Create {
            user,
            item_type,
            item_id,
            comment,
        } => {
            let username = resolve_username(&client, user).await?;

            let mut post = BookmarkPost::new()
                .type_name(item_type)
                .favorited_id(*item_id);
            if let Some(c) = comment {
                post = post.comment(c);
            }

            let response = client.favorites().create(&username, &post).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Created favorite: {}", response.favorite.id);
            }
        }

        FavoriteCommands::Delete { user, id } => {
            let username = resolve_username(&client, user).await?;
            client.favorites().delete(&username, *id).await?;
            println!("Deleted favorite: {}", id);
        }
    }

    Ok(())
}

async fn run_bundles_command(cli: &Cli, cmd: &BundleCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        BundleCommands::List {
            user,
            page,
            page_size,
        } => {
            let username = resolve_username(&client, user).await?;
            let params = BundlesListParams::new().page(*page).page_size(*page_size);
            let response = client.bundles().list(&username, &params).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                if let Some(paginator) = &response.paginator {
                    println!(
                        "Bundles (page {}/{}):",
                        paginator.page, paginator.page_count
                    );
                } else {
                    println!("Bundles:");
                }
                for bundle in &response.bundles {
                    let name = bundle.name.as_deref().unwrap_or("Unnamed");
                    let count = bundle.bundled_items_count.unwrap_or(0);
                    println!("  {} - {} ({} items)", bundle.id, name, count);
                }
            }
        }

        BundleCommands::Show { user, id } => {
            let username = resolve_username(&client, user).await?;
            let response = client.bundles().show(&username, *id).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                let bundle = &response.bundle;
                println!("Bundle: {}", bundle.id);
                if let Some(name) = &bundle.name {
                    println!("Name: {name}");
                }
                if let Some(count) = bundle.bundled_items_count {
                    println!("Items: {count}");
                }
                if let Some(public) = bundle.is_public {
                    println!("Public: {}", if public { "Yes" } else { "No" });
                }
            }
        }

        BundleCommands::Create { user, name, public } => {
            let username = resolve_username(&client, user).await?;

            let post = BundlePost::new().name(name).is_public(*public);
            let response = client.bundles().create(&username, &post).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Created bundle: {} (ID: {})", name, response.bundle.id);
            }
        }

        BundleCommands::Delete { user, id } => {
            let username = resolve_username(&client, user).await?;
            client.bundles().delete(&username, *id).await?;
            println!("Deleted bundle: {}", id);
        }
    }

    Ok(())
}

async fn run_friends_command(cli: &Cli, cmd: &FriendCommands) -> Result<(), CliError> {
    let client = cli.build_client().await?;

    match cmd {
        FriendCommands::List { user } => {
            let username = resolve_username(&client, user).await?;
            let response = client.friends().list(&username).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Friends:");
                for friendship in &response.friendships {
                    let friend_name = friendship
                        .friend
                        .as_ref()
                        .map(|f| f.username.as_str())
                        .unwrap_or("Unknown");
                    let mutual = if friendship.mutual.unwrap_or(false) {
                        " [mutual]"
                    } else {
                        ""
                    };
                    println!("  {} - {}{}", friendship.id, friend_name, mutual);
                }
            }
        }

        FriendCommands::Activity {
            user,
            page,
            page_size,
        } => {
            let username = resolve_username(&client, user).await?;
            let params = FriendsActivityParams::new()
                .page(*page)
                .page_size(*page_size);
            let response = client.friends().activity(&username, &params).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Friend Activity:");
                for activity in &response.activity {
                    let activity_type = activity.activity_type.as_deref().unwrap_or("unknown");
                    let user_name = activity
                        .user
                        .as_ref()
                        .map(|u| u.username.as_str())
                        .unwrap_or("Unknown");
                    println!("  [{activity_type}] by {user_name}");
                }
            }
        }

        FriendCommands::Add {
            user,
            friend_user_id,
        } => {
            let username = resolve_username(&client, user).await?;
            let response = client.friends().create(&username, *friend_user_id).await?;

            if cli.json_output() {
                cli.print_json(&response)?;
            } else {
                println!("Added friend! Friendship ID: {}", response.friendship.id);
            }
        }

        FriendCommands::Remove {
            user,
            friendship_id,
        } => {
            let username = resolve_username(&client, user).await?;
            client.friends().destroy(&username, *friendship_id).await?;
            println!("Removed friendship: {}", friendship_id);
        }
    }

    Ok(())
}

/// Wait for OAuth callback over HTTPS and extract the authorization code.
///
/// Generates a self-signed certificate for localhost at runtime.
/// The browser will show a security warning that users need to accept.
async fn wait_for_oauth_callback() -> Result<String, CliError> {
    use rcgen::{generate_simple_self_signed, CertifiedKey};
    use rustls_pemfile::{certs, private_key};
    use std::io::Cursor;
    use std::sync::Arc;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpListener;
    use tokio_rustls::rustls::crypto::aws_lc_rs;
    use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
    use tokio_rustls::rustls::ServerConfig;
    use tokio_rustls::TlsAcceptor;

    // Install the crypto provider (required for rustls 0.23+)
    let _ = aws_lc_rs::default_provider().install_default();

    // Generate a self-signed certificate for localhost
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names)
        .map_err(|e| CliError::Other(format!("Failed to generate certificate: {e}")))?;

    // Convert to PEM format and parse back for rustls
    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    let cert_chain: Vec<CertificateDer<'static>> = certs(&mut Cursor::new(cert_pem.as_bytes()))
        .filter_map(|r| r.ok())
        .collect();

    let key: PrivateKeyDer<'static> = private_key(&mut Cursor::new(key_pem.as_bytes()))
        .map_err(|e| CliError::Other(format!("Failed to parse private key: {e}")))?
        .ok_or_else(|| CliError::Other("No private key found".to_string()))?;

    // Build TLS config
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .map_err(|e| CliError::Other(format!("Failed to build TLS config: {e}")))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    // Bind to localhost (not 127.0.0.1) for HTTPS
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("\n⚠️  Your browser will show a security warning about the self-signed certificate.");
    println!("   Click 'Advanced' → 'Proceed to localhost' to continue.\n");

    // Loop to handle TLS handshake failures (browser may retry after user accepts cert)
    let mut tls_stream = loop {
        let (tcp_stream, _) = listener.accept().await?;

        // Perform TLS handshake - retry on failure
        match acceptor.accept(tcp_stream).await {
            Ok(stream) => break stream,
            Err(e) => {
                eprintln!("   (TLS handshake attempt failed: {e} - waiting for retry...)");
                continue;
            }
        }
    };

    // Read the HTTP request
    let (reader, mut writer) = tokio::io::split(&mut tls_stream);
    let mut reader = BufReader::new(reader);

    let mut request_line = String::new();
    reader.read_line(&mut request_line).await?;

    // Parse the GET request to extract the code
    // Expected format: GET /callback?code=XXX&state=YYY HTTP/1.1
    let code = request_line
        .split_whitespace()
        .nth(1)
        .and_then(|path| {
            path.strip_prefix("/callback?")
                .or_else(|| path.strip_prefix("/callback/?"))
        })
        .and_then(|query| {
            query
                .split('&')
                .find_map(|param| param.strip_prefix("code=").map(|code| code.to_string()))
        })
        .ok_or(CliError::MissingCredentials(
            "No authorization code received in callback",
        ))?;

    // Send response
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\
        <html><body style='font-family: system-ui; text-align: center; padding: 50px;'>\
        <h1>✅ Authorization successful!</h1>\
        <p>You can close this window and return to the terminal.</p>\
        </body></html>";
    writer.write_all(response.as_bytes()).await?;

    Ok(code)
}
