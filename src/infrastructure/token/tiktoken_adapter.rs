use tiktoken_rs::{CoreBPE, get_bpe_from_tokenizer, tokenizer::Tokenizer};

use crate::{
    application::services::TokenCounterPort, domain::common::token::TokenCount,
};

pub struct TiktokenAdapter {
    bpe: CoreBPE,
}

impl TiktokenAdapter {
    pub fn new() -> Self {
        let bpe = get_bpe_from_tokenizer(Tokenizer::Cl100kBase).unwrap();
        Self { bpe }
    }
}

impl TokenCounterPort for TiktokenAdapter {
    fn count(&self, text: &str) -> TokenCount {
        let tokens = self.bpe.encode_ordinary(text);
        TokenCount::new(tokens.len() as u32)
    }
}
