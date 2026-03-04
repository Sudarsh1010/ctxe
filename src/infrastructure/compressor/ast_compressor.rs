use itertools::intersperse;
use tiktoken_rs::CoreBPE;

use crate::{
    domain::{
        code::service::compressor::{
            CompressionLevel, CompressionResult, Compressor,
        },
        common::token::TokenBudget,
    },
    infrastructure::token::shared::get_shared_tokenizer,
};

pub struct AstCompressor {
    bpe: CoreBPE,
}

impl AstCompressor {
    pub fn new() -> crate::Result<Self> {
        let bpe = get_shared_tokenizer().clone();
        Ok(Self { bpe })
    }

    /// Count tokens using the configured tokenizer
    fn count_tokens(&self, text: &str) -> u32 {
        self.bpe.encode_ordinary(text).len() as u32
    }
}

impl Compressor for AstCompressor {
    fn compress(
        &self,
        code: &str,
        symbols: &[crate::domain::code::value::symbol::Symbol],
        level: crate::domain::code::service::compressor::CompressionLevel,
        budget: Option<TokenBudget>,
    ) -> crate::Result<CompressionResult> {
        let compressed = match level {
            CompressionLevel::SignaturesOnly => intersperse(
                symbols.iter().map(|s| s.signature.as_str()),
                "\n\n",
            )
            .collect::<String>(),

            CompressionLevel::WithDocs => intersperse(
                symbols.iter().map(|s| {
                    s.doc
                        .as_ref()
                        .map(|doc| format!("{doc}\n{}", s.signature))
                        .unwrap_or_else(|| s.signature.clone())
                }),
                "\n\n".to_string(),
            )
            .collect::<String>(),

            CompressionLevel::RemoveComments => remove_comments(code),
            CompressionLevel::None => code.to_string(),
        };

        let original_tokens = self.count_tokens(code);
        let compressed_tokens = self.count_tokens(&compressed);

        let result = CompressionResult::new(
            compressed,
            original_tokens,
            compressed_tokens,
        );

        if let Some(budget) = budget
            && result.compressed_tokens > budget.as_u32()
        {
            return Err(crate::Error::TokenBudget {
                current: result.compressed_tokens as usize,
                budget: budget.as_u32() as usize,
            });
        }

        Ok(result)
    }
}

/// Removes line comments only.
///
/// **Limitations**: Does not handle block comments, comments in strings,
/// or nested comments. For production use, consider using tree-sitter
/// to properly identify comment nodes.
fn remove_comments(code: &str) -> String {
    // Very basic: remove // comments
    intersperse(
        code.lines().map(|line| {
            if let Some(idx) = line.find("//") {
                &line[..idx]
            } else {
                line
            }
        }),
        "\n",
    )
    .collect::<String>()
}
