//! Configuration and profile management for the Ravelry CLI.
//!
//! Stores authentication profiles in `~/.config/ravelry/config.toml`.

use directories::ProjectDirs;
use ravelry::{OAuth2Token, RavelryError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// CLI configuration containing authentication profiles.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    /// The currently active profile name.
    #[serde(default)]
    pub current_profile: Option<String>,

    /// Available authentication profiles.
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

/// An authentication profile.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Profile {
    /// HTTP Basic authentication with access key + personal key.
    Basic {
        access_key: String,
        personal_key: String,
    },
    /// OAuth2 authentication with stored tokens.
    OAuth2 {
        client_id: String,
        client_secret: String,
        token: OAuth2Token,
    },
}

impl Profile {
    /// Create a new Basic auth profile.
    pub fn basic(access_key: impl Into<String>, personal_key: impl Into<String>) -> Self {
        Self::Basic {
            access_key: access_key.into(),
            personal_key: personal_key.into(),
        }
    }

    /// Create a new OAuth2 profile.
    pub fn oauth2(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        token: OAuth2Token,
    ) -> Self {
        Self::OAuth2 {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            token,
        }
    }
}

impl Config {
    /// Get the path to the config file.
    pub fn path() -> Option<PathBuf> {
        ProjectDirs::from("com", "ravelry", "ravelry-cli")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }

    /// Load the configuration from disk.
    ///
    /// Returns a default empty config if the file doesn't exist.
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::path().ok_or(ConfigError::NoConfigDir)?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save the configuration to disk.
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::path().ok_or(ConfigError::NoConfigDir)?;

        // Ensure the config directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Get the current profile.
    #[allow(dead_code)]
    pub fn current_profile(&self) -> Option<&Profile> {
        self.current_profile
            .as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    /// Get a profile by name.
    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    /// Set or update a profile.
    pub fn set_profile(&mut self, name: impl Into<String>, profile: Profile) {
        self.profiles.insert(name.into(), profile);
    }

    /// Set the current profile name.
    pub fn set_current(&mut self, name: impl Into<String>) {
        self.current_profile = Some(name.into());
    }

    /// Delete a profile.
    #[allow(dead_code)]
    pub fn delete_profile(&mut self, name: &str) -> Option<Profile> {
        let removed = self.profiles.remove(name);

        // Clear current if we just deleted it
        if self.current_profile.as_deref() == Some(name) {
            self.current_profile = None;
        }

        removed
    }

    /// List all profile names.
    pub fn profile_names(&self) -> Vec<&str> {
        self.profiles.keys().map(|s| s.as_str()).collect()
    }
}

/// Errors that can occur during config operations.
#[derive(Debug)]
pub enum ConfigError {
    /// Could not determine the config directory.
    NoConfigDir,
    /// I/O error reading/writing the config file.
    Io(std::io::Error),
    /// Error parsing the config file.
    Parse(toml::de::Error),
    /// Error serializing the config.
    Serialize(toml::ser::Error),
    /// API error (for token refresh, etc.).
    Api(RavelryError),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoConfigDir => write!(f, "Could not determine config directory"),
            Self::Io(e) => write!(f, "Config I/O error: {e}"),
            Self::Parse(e) => write!(f, "Config parse error: {e}"),
            Self::Serialize(e) => write!(f, "Config serialize error: {e}"),
            Self::Api(e) => write!(f, "API error: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        Self::Parse(e)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(e: toml::ser::Error) -> Self {
        Self::Serialize(e)
    }
}

impl From<RavelryError> for ConfigError {
    fn from(e: RavelryError) -> Self {
        Self::Api(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_profile_serialization() {
        let profile = Profile::basic("access", "personal");

        let toml = toml::to_string(&profile).unwrap();
        assert!(toml.contains("type = \"Basic\""));
        assert!(toml.contains("access_key = \"access\""));
    }

    #[test]
    fn test_config_roundtrip() {
        let mut config = Config::default();
        config.set_profile("test", Profile::basic("key", "secret"));
        config.set_current("test");

        let toml = toml::to_string(&config).unwrap();
        let loaded: Config = toml::from_str(&toml).unwrap();

        assert_eq!(loaded.current_profile, Some("test".to_string()));
        assert!(loaded.profiles.contains_key("test"));
    }
}
