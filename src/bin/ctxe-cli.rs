use std::path::PathBuf;

use clap::Parser;
use ctxe::{
    VERSION,
    application::commands::{CompressCommand, handle_compress},
    build_services,
    domain::{
        code::service::compressor::CompressionLevel, common::language::Language,
    },
};

#[derive(clap::ValueEnum, Clone, Copy)]
enum CompressionLevelArg {
    Signatures,
    WithDocs,
    RemoveComments,
    None,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Compress code files
    Compress {
        files: Vec<PathBuf>,

        #[arg(short, long, default_value = "signatures")]
        level: CompressionLevelArg,

        #[arg(short, long)]
        budget: Option<u32>,
    },

    /// Count tokens in text
    Tokens {
        /// Text to count (or use stdin)
        #[arg(short, long)]
        input: Option<String>,

        #[arg(short, long)]
        file: Vec<PathBuf>,

        #[arg(short, long)]
        budget: Option<u32>,
    },
}

impl From<CompressionLevelArg> for CompressionLevel {
    fn from(arg: CompressionLevelArg) -> Self {
        match arg {
            CompressionLevelArg::Signatures => Self::SignaturesOnly,
            CompressionLevelArg::WithDocs => Self::WithDocs,
            CompressionLevelArg::RemoveComments => Self::RemoveComments,
            CompressionLevelArg::None => Self::None,
        }
    }
}

#[derive(Parser)]
#[command(name = "ctxe")]
#[command(about = "Context Engineer for AI Coding Agents")]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut services = build_services()?;

    match cli.command {
        Commands::Compress {
            files,
            level,
            budget,
        } => {
            for file in files {
                let code = std::fs::read_to_string(&file)?;
                let language = Language::from_extension(&file);

                let cmd = CompressCommand {
                    code,
                    language,
                    level: level.into(),
                    budget,
                };

                let result = handle_compress(
                    cmd,
                    &mut services.parser_service,
                    &services.compressor_service,
                )?;

                println!("{}", result.compressed);
                if cli.verbose {
                    eprintln!(
                        "Compressed: {} → {} tokens ({:.1}% reduction)",
                        result.original_tokens,
                        result.compressed_tokens,
                        result.reduction_percent
                    );
                }
            }
        }

        Commands::Tokens {
            input,
            file,
            budget,
        } => {
            let text = if let Some(input) = input {
                input
            } else {
                file.iter()
                    .map(std::fs::read_to_string)
                    .collect::<std::result::Result<Vec<_>, _>>()?
                    .join("\n")
            };

            let count = services.token_counter_service.count(&text);

            if let Some(budget) = budget {
                let budget_obj =
                    ctxe::domain::common::token::TokenBudget::new(budget)?;
                if count.exceeds(budget_obj) {
                    eprintln!(
                        "⚠️  Token count ({}) exceeds budget ({})",
                        count.as_u32(),
                        budget
                    );
                }
            }

            println!("Tokens: {}", count.as_u32());
        }
    }

    Ok(())
}
