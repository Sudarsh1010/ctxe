use tiktoken_rs::{CoreBPE, get_bpe_from_tokenizer, tokenizer::Tokenizer};

use crate::domain::{
    code::service::compressor::{
        CompressionLevel, CompressionResult, Compressor,
    },
    common::token::TokenBudget,
};

pub struct AstCompressor {
    bpe: CoreBPE,
}

impl AstCompressor {
    pub fn new() -> crate::Result<Self> {
        let bpe = get_bpe_from_tokenizer(Tokenizer::Cl100kBase).unwrap();
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
            CompressionLevel::SignaturesOnly => symbols
                .iter()
                .map(|s| s.signature.clone())
                .collect::<Vec<_>>()
                .join("\n\n")
                .to_string(),

            CompressionLevel::WithDocs => symbols
                .iter()
                .map(|s| {
                    let mut out = String::new();
                    if let Some(doc) = &s.doc {
                        out.push_str(doc);
                        out.push('\n');
                    }
                    out.push_str(&s.signature);
                    out
                })
                .collect::<Vec<_>>()
                .join("\n\n"),

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

fn remove_comments(code: &str) -> String {
    // Very basic: remove // and /* */ comments
    // For production, use tree-sitter to properly identify comment nodes
    code.lines()
        .map(|line| {
            if let Some(idx) = line.find("//") {
                &line[..idx]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
