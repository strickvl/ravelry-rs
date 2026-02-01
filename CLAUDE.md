# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**ravelry-rs** is a Rust workspace containing two crates:
- `ravelry` - A typed, async client library for the [Ravelry API](https://www.ravelry.com/api) (knitting/crochet community)
- `ravelry-cli` - Command-line interface that wraps the library

## Build & Test Commands

```bash
# Build everything
cargo build

# Run all tests (unit, integration, and doc tests)
cargo test

# Run a single test by name
cargo test test_name

# Run tests for a specific crate
cargo test -p ravelry
cargo test -p ravelry-cli

# Run only integration tests
cargo test --test upload_tests
cargo test --test error_tests
cargo test --test messages_tests
cargo test --test favorites_tests

# Lint with clippy
cargo clippy --all-targets

# Format code
cargo fmt

# Run the CLI
cargo run -p ravelry-cli -- <subcommand>
```

## Architecture

### Service Pattern for API Endpoints

The library uses a service pattern where `RavelryClient` exposes namespaced API groups:

```rust
client.patterns().search(&params)      // PatternsApi
client.yarns().search(&params)         // YarnsApi
client.projects().list(user, &params)  // ProjectsApi
client.stash().list(user, &params)     // StashApi
client.messages().list(&params)        // MessagesApi
client.upload().image(token, files)    // UploadApi (Tier 2)
client.favorites().list(user, &params) // FavoritesApi (Tier 2)
client.bundles().list(user, &params)   // BundlesApi (Tier 2)
client.friends().list(user)            // FriendsApi (Tier 2)
client.root().current_user()           // RootApi
```

Each service struct holds a reference to the client and provides methods for related endpoints.

### Type Variants Strategy

The API returns different field sets depending on context. We use separate structs:
- `PatternList` / `PatternFull` - Search results vs single entity
- `BookmarkList` / `BookmarkFull` / `BookmarkPost` - Favorites
- `BundleList` / `BundleFull` / `BundlePost` - Bundles
- Similar patterns for Project, Stash, Message, etc.

### Authentication

Three auth strategies implement the `Authenticator` trait:
- `BasicAuth` - HTTP Basic auth (access key + personal key)
- `OAuth2Auth` - Bearer token from OAuth2 flow
- `NoAuth` - For unauthenticated requests

**Special case:** The upload API (`/upload/image.json`) is unauthenticated per Ravelry docs. The client uses an internal `AuthMode::None` for these endpoints.

The CLI stores credentials in `~/.config/ravelry/config.toml` as named profiles.

### Error Handling

`RavelryError` provides typed variants for API errors, rate limiting (429), ETag-based caching (304), and invalid request validation.

## Key Files

| Path | Purpose |
|------|---------|
| `crates/ravelry/src/client/mod.rs` | `RavelryClient` builder and request helpers, AuthMode |
| `crates/ravelry/src/auth/oauth2.rs` | OAuth2 flow with HTTPS callback server |
| `crates/ravelry/src/api/*.rs` | Endpoint implementations (service pattern) |
| `crates/ravelry/src/types/*.rs` | Response/request structs |
| `crates/ravelry/tests/*.rs` | Wiremock integration tests |
| `crates/ravelry-cli/src/main.rs` | CLI command definitions and handlers |
| `crates/ravelry-cli/src/config.rs` | Profile/token storage |

## Implementation Status

**Completed through Phase 4** (see `design/IMPLEMENTATION_PLAN.md` for full roadmap).

**Tier 1 (Core):**
- Client foundation with Basic and OAuth2 auth
- Patterns: search, show, projects
- Yarns: search, show
- Projects: list, show, create, update, delete
- Stash: list, show, create, update, delete
- Messages: list, show, create, reply, mark_read/unread, archive/unarchive, delete
- Root: current_user

**Tier 2 (Community):**
- Upload: request_token, image (multipart), image_status
- Favorites: list, show, create, update, delete, add_to_bundle, remove_from_bundle
- Bundles: list, show, create, update, delete
- Bundled Items: show, delete
- Friends: list, activity, create, destroy

**CLI Commands:**
- `auth login/basic/profiles/use/delete/refresh/whoami`
- `patterns search/show/projects`
- `yarns search/show`
- `projects list/show/create/update`
- `stash list/show/create/update`
- `messages list/read/send/reply/mark-read/mark-unread/archive/unarchive/delete`
- `upload image`
- `favorites list/show/create/delete`
- `bundles list/show/create/delete`
- `friends list/activity/add/remove`

**Not yet implemented:**
- Tier 3 endpoints (commerce/pro features)
- OAuth1a (3-legged) authentication
