// mod commands;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ctxe")]
#[command(about = "Context engineering for AI coding agents")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan codebase and build index
    Scan { path: PathBuf },

    /// Query codebase semantically
    Query { text: String },

    /// Compress files
    Compress { files: Vec<PathBuf> },

    /// Count tokens
    Tokens { input: String },

    /// Start MCP server
    ServeMcp,
}

pub fn parse(cli: Cli) -> crate::Result<()> {
    // match cli.command {
    // Commands::Scan { path } => commands::scan(path),
    // Commands::Query { text } => commands::query(text),
    // Commands::Compress { files } => commands::compress(files),
    // Commands::Tokens { input } => commands::tokens(input),
    // Commands::ServeMcp => commands::serve_mcp(),
    // }
}
