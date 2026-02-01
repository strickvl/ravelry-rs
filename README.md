# ravelry-rs

A typed, async Rust client library and CLI for the [Ravelry API](https://www.ravelry.com/api).

Ravelry is a community site for knitters and crocheters, and this library provides programmatic access to patterns, yarns, projects, stash, messages, and more.

## Features

- **Fully typed** - All API responses are deserialized into Rust structs
- **Async/await** - Built on `tokio` and `reqwest` for async HTTP
- **Multiple auth methods** - Basic auth, OAuth2 with refresh tokens
- **CLI included** - Command-line tool for interactive use
- **Comprehensive coverage** - Patterns, yarns, projects, stash, messages, uploads, favorites, bundles, friends

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ravelry = { git = "https://github.com/strickvl/ravelry-rs" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

### Basic Authentication

```rust
use ravelry::{RavelryClient, auth::BasicAuth};

#[tokio::main]
async fn main() -> Result<(), ravelry::RavelryError> {
    // Create client with Basic auth (access key + personal key from Ravelry)
    let auth = BasicAuth::new("your_access_key", "your_personal_key");
    let client = RavelryClient::builder(auth).build()?;

    // Get current user
    let user = client.root().current_user().await?;
    println!("Logged in as: {}", user.user.username);

    // Search for patterns
    use ravelry::api::patterns::PatternSearchParams;
    let params = PatternSearchParams::new()
        .query("baby blanket")
        .page_size(5);
    let results = client.patterns().search(&params).await?;

    for pattern in results.patterns {
        println!("{}: {}", pattern.id, pattern.name);
    }

    Ok(())
}
```

### OAuth2 Authentication

```rust
use ravelry::{RavelryClient, auth::OAuth2Auth, RavelryOAuth2Client};

#[tokio::main]
async fn main() -> Result<(), ravelry::RavelryError> {
    // For OAuth2 flow, first obtain tokens via the OAuth2 client
    let oauth = RavelryOAuth2Client::new(
        "client_id",
        "client_secret",
        "https://localhost:8080/callback"
    )?;

    // Generate auth URL and complete flow (see CLI for full example)
    let (auth_url, _state) = oauth.authorize_url(vec!["offline".to_string()]);
    println!("Open: {}", auth_url);

    // After receiving the callback code...
    // let token = oauth.exchange_code(&code).await?;

    // Create client with obtained token
    let auth = OAuth2Auth::new("access_token_here");
    let client = RavelryClient::builder(auth).build()?;

    Ok(())
}
```

## API Coverage

### Tier 1 (Core)

| API | Methods |
|-----|---------|
| Patterns | `search`, `show`, `projects` |
| Yarns | `search`, `show` |
| Projects | `list`, `show`, `create`, `update`, `delete` |
| Stash | `list`, `show`, `create`, `update`, `delete` |
| Messages | `list`, `show`, `create`, `reply`, `mark_read`, `mark_unread`, `archive`, `unarchive`, `delete` |
| Root | `current_user` |

### Tier 2 (Community)

| API | Methods |
|-----|---------|
| Upload | `request_token`, `image`, `image_status` |
| Favorites | `list`, `show`, `create`, `update`, `delete`, `add_to_bundle`, `remove_from_bundle` |
| Bundles | `list`, `show`, `create`, `update`, `delete` |
| Bundled Items | `show`, `delete` |
| Friends | `list`, `activity`, `create`, `destroy` |

## CLI Usage

The CLI provides a convenient way to interact with the Ravelry API.

### Setup

```bash
# Build the CLI
cargo build --release -p ravelry-cli

# Or run directly
cargo run -p ravelry-cli -- <command>
```

### Authentication

```bash
# OAuth2 login (recommended for full access)
ravelry auth login --client-id YOUR_ID --client-secret YOUR_SECRET

# Or use basic auth
ravelry auth basic --access-key KEY --personal-key KEY

# Check who you're logged in as
ravelry whoami
```

### Examples

```bash
# Search patterns
ravelry patterns search --query "sock" --craft knitting --page-size 10

# Show pattern details
ravelry patterns show 123456

# List your projects
ravelry projects list

# Create a project
ravelry projects create --name "My Sweater" --pattern-id 12345

# List your stash
ravelry stash list --all

# Send a message
ravelry messages send --to "username" --subject "Hi!" --content "Hello there"

# Upload images
ravelry upload image photo1.jpg photo2.jpg

# List favorites
ravelry favorites list --type-filter pattern

# Check friend activity
ravelry friends activity
```

### Global Flags

- `--profile <name>` - Use a specific auth profile
- `--json` - Output as JSON
- `--json-pretty` - Output as pretty-printed JSON
- `--debug` - Enable API debug mode

## Error Handling

The library provides typed errors for different scenarios:

```rust
use ravelry::RavelryError;

match client.patterns().show(999999).await {
    Ok(response) => println!("Found: {}", response.pattern.name),
    Err(RavelryError::RateLimited { retry_after, .. }) => {
        println!("Rate limited! Retry after {:?}", retry_after);
    }
    Err(RavelryError::NotModified { etag }) => {
        println!("Content unchanged (ETag: {:?})", etag);
    }
    Err(RavelryError::ApiStatus { status, body }) => {
        println!("API error {}: {:?}", status, body);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Upload Example

The upload flow requires three steps:

```rust
use ravelry::types::UploadFile;

// 1. Request an upload token
let token_resp = client.upload().request_token().await?;

// 2. Read file and upload
let bytes = std::fs::read("photo.jpg")?;
let file = UploadFile::new("photo.jpg", bytes);

let upload_resp = client.upload()
    .image(&token_resp.upload_token, vec![file])
    .await?;

// 3. Check status (optional)
let status = client.upload()
    .image_status(&token_resp.upload_token)
    .await?;

for upload in status.uploads {
    for (key, result) in upload {
        println!("{}: image_id = {}", key, result.image_id);
    }
}
```

## Development

```bash
# Run tests
cargo test

# Run clippy
cargo clippy --all-targets

# Format code
cargo fmt
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
