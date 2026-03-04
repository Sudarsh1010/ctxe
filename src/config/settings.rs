//! Settings struct with env var support

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Default compression level
    #[serde(default)]
    pub default_compression: CompressionConfig,

    /// Logging verbosity
    #[serde(default)]
    pub verbose: bool,
}

impl Settings {
    /// Load settings with precedence: env vars > config file > defaults
    pub fn load() -> Result<Self, ConfigError> {
        // Try to load from config file first
        let config_path = std::env::var("CTXE_CONFIG")
            .ok()
            .map(PathBuf::from)
            .or_else(|| {
                // Default locations
                dirs::config_dir().map(|d| d.join("ctxe").join("config.toml"))
            });

        let builder = if let Some(path) = config_path {
            if path.exists() {
                config::Config::builder().add_source(config::File::from(path))
            } else {
                config::Config::builder()
            }
        } else {
            config::Config::builder()
        };

        // Override with env vars (CTXE_*)
        let builder = builder.add_source(
            config::Environment::with_prefix("CTXE").separator("_"),
        );

        // Build and deserialize
        let config = builder
            .build()
            .map_err(|e| ConfigError::Load(e.to_string()))?;

        config
            .try_deserialize()
            .map_err(|e| ConfigError::Parse(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    #[serde(default = "default_compression_level")]
    pub level: String, // "signatures", "with_docs", etc.
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            level: default_compression_level(),
        }
    }
}

fn default_compression_level() -> String {
    "signatures".to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to load config: {0}")]
    Load(String),
    #[error("Failed to parse config: {0}")]
    Parse(String),
}
