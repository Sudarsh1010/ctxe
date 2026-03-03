use tiktoken_rs::tokenizer::Tokenizer;
use tiktoken_rs::{CoreBPE, get_bpe_from_model, get_bpe_from_tokenizer};

use crate::domain::common::token::TokenCount;

pub struct TokenCounter {
    bpe: CoreBPE,
}

impl Default for TokenCounter {
    fn default() -> Self {
        let bpe = get_bpe_from_tokenizer(Tokenizer::Cl100kBase).unwrap();
        Self { bpe }
    }
}

impl TokenCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self, text: &str) -> TokenCount {
        let tokens = self.bpe.encode_ordinary(text);
        TokenCount::new(tokens.len() as u32)
    }

    pub fn count_with_model(&self, text: &str, model: &str) -> crate::error::Result<TokenCount> {
        let bpe = get_bpe_from_model(model)
            .map_err(|e| crate::error::Error::Parse(format!("Invalid model: {e}")))?;
        let tokens = bpe.encode_ordinary(text);
        Ok(TokenCount::new(tokens.len() as u32))
    }
}
