use std::io::stdin;

use clap::Parser;
use ctxe::{TokenCountCommand, TokenCounter, VERSION, handle_token_count};

#[derive(clap::Subcommand)]
enum Commands {
    /// Count tokens in text
    Tokens {
        /// Text to count (or use stdin)
        input: Option<String>,
    },
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

    match cli.command {
        Commands::Tokens { input } => {
            let text = match input {
                Some(t) => t,
                None => {
                    use std::io::Read;
                    let mut buffer = String::new();
                    stdin().read_to_string(&mut buffer)?;
                    buffer
                }
            };

            let counter = TokenCounter::new();
            let cmd = TokenCountCommand::new(text);
            let result = handle_token_count(cmd, &counter)?;

            println!("Tokens: {}", result.token_count.as_u32());
            if cli.verbose {
                println!("Preview: {}", result.text_preview);
            }

            Ok(())
        }
    }
}
