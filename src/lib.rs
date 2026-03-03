//! ctx-engine: Dynamic context engineering for AI coding agents
//!
//! This library provides core functionality for:
//! - Code parsing (tree-sitter)
//! - Context compression
//! - Token counting
//! - Git-aware file selection
//! - Semantic search (Phase 2)

pub mod application;
pub mod domain;
pub mod error;

pub use application::commands::{TokenCountCommand, TokenCountResult, handle_token_count};
pub use application::services::TokenCounter;
pub use domain::common::token::{TokenBudget, TokenCount};
pub use error::{Error, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
