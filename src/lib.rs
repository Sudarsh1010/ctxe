//! ctx-engine: Dynamic context engineering for AI coding agents
//!
//! This library provides core functionality for:
//! - Code parsing (tree-sitter)
//! - Context compression
//! - Token counting
//! - Git-aware file selection
//! - Semantic search (Phase 2)

pub mod application;
pub mod config;
pub mod domain;
pub mod error;
pub mod infrastructure;

pub use error::{Error, Result};

use crate::{
    application::services::{
        CompressorService, ParserService, TokenCounterService,
    },
    infrastructure::{
        compressor::ast_compressor::AstCompressor,
        parsing::tree_sitter_adapter::TreeSitterParser,
        token::tiktoken_adapter::TiktokenAdapter,
    },
};

/// Build all application services with production infrastructure.
///
/// This is the **composition root** — the only place that knows about
/// concrete infrastructure types. Everything else depends on traits.
pub fn build_services() -> Result<AppComponent> {
    // === Infrastructure Adapters (concrete implementations) ===
    let parser = TreeSitterParser::new()?;
    let token_counter = TiktokenAdapter::new()?;
    let compressor = AstCompressor::new()?;

    // === Application Services (orchestration) ===
    let parser_service = ParserService::new(parser);
    let token_counter_service = TokenCounterService::new(token_counter);
    let compressor_service = CompressorService::new(compressor);

    Ok(AppComponent {
        parser_service,
        token_counter_service,
        compressor_service,
    })
}

/// Container for wired services.
///
/// Returned by `build_services()` and passed to command handlers.
/// Entry points (CLI/MCP) use this to access functionality.
pub struct AppComponent {
    parser_service: ParserService<TreeSitterParser>,
    token_counter_service: TokenCounterService<TiktokenAdapter>,
    compressor_service: CompressorService<AstCompressor>,
}

impl AppComponent {
    /// Provides mutable access to all services simultaneously.
    ///
    /// This pattern enables disjoint borrows while keeping fields private.
    pub fn with_services<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(
            &mut ParserService<TreeSitterParser>,
            &mut CompressorService<AstCompressor>,
            &mut TokenCounterService<TiktokenAdapter>,
        ) -> R,
    {
        f(
            &mut self.parser_service,
            &mut self.compressor_service,
            &mut self.token_counter_service,
        )
    }
}

// ─────────────────────────────────────────────────────────────
// Crate Metadata
// ─────────────────────────────────────────────────────────────
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
