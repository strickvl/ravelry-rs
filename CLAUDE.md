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

# Run all tests
cargo test

# Run a single test by name
cargo test test_name

# Run tests for a specific crate
cargo test -p ravelry
cargo test -p ravelry-cli

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
client.patterns().search(&params)   // PatternsApi
client.yarns().search(&params)      // YarnsApi
client.projects().list(user, &params)  // ProjectsApi
client.root().current_user()        // RootApi
```

Each service struct (`PatternsApi`, `YarnsApi`, etc.) holds a reference to the client and provides methods for related endpoints.

### Type Variants Strategy

The API returns different field sets depending on context. We use separate structs:
- `PatternList` - Minimal fields in search results
- `PatternFull` - All fields when fetching a single entity (not yet implemented)
- `PatternPost` - Writable fields for create/update (not yet implemented)

This pattern applies to all major entity types (Pattern, Yarn, Project, Stash, Message).

### Authentication

Three auth strategies implement the `Authenticator` trait:
- `BasicAuth` - HTTP Basic auth (access key + personal key)
- `OAuth2Auth` - Bearer token from OAuth2 flow
- `NoAuth` - For unauthenticated requests

The CLI stores credentials in `~/.config/ravelry/config.toml` as named profiles.

### Error Handling

`RavelryError` provides typed variants for API errors, rate limiting (429), and ETag-based caching (304).

## Key Files

| Path | Purpose |
|------|---------|
| `crates/ravelry/src/client/mod.rs` | `RavelryClient` builder and request helpers |
| `crates/ravelry/src/auth/oauth2.rs` | OAuth2 flow with HTTPS callback server |
| `crates/ravelry/src/api/*.rs` | Endpoint implementations (service pattern) |
| `crates/ravelry/src/types/*.rs` | Response/request structs |
| `crates/ravelry-cli/src/config.rs` | Profile/token storage |

## Implementation Status

Currently implementing **Phase 2** (OAuth + Tier 1 endpoints). See `design/IMPLEMENTATION_PLAN.md` for the full roadmap.

**Completed:**
- Client foundation with Basic and OAuth2 auth
- Pattern, Yarn, Project, Stash, Message search/list endpoints
- CLI with OAuth2 browser flow (HTTPS callback with self-signed certs)
- Profile-based config storage

**Not yet implemented:**
- Single entity fetch (`/patterns/{id}.json`, etc.)
- Create/update/delete operations
- Upload support
- Tier 2+ endpoints (favorites, bundles, forums)
