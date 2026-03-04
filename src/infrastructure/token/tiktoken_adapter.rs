use tiktoken_rs::CoreBPE;

use crate::{
    application::services::TokenCounterPort, domain::common::token::TokenCount,
    infrastructure::token::shared::get_shared_tokenizer,
};

pub struct TiktokenAdapter {
    bpe: CoreBPE,
}

impl TiktokenAdapter {
    pub fn new() -> crate::Result<Self> {
        let bpe = get_shared_tokenizer().clone();
        Ok(Self { bpe })
    }
}

impl TokenCounterPort for TiktokenAdapter {
    fn count(&self, text: &str) -> TokenCount {
        let tokens = self.bpe.encode_ordinary(text);
        TokenCount::new(tokens.len() as u32)
    }
}
