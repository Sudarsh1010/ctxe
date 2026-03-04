//! Heuristic-based token counter adapter.
//!
//! Uses a simple character-based heuristic that works for all models
//! (OpenAI, Claude, GLM, Qwen, Kimi, etc.) with ±10-15% accuracy.

use crate::{
    application::services::TokenCounterPort, domain::common::token::TokenCount,
};

pub struct HeuristicTokenCounter;

impl HeuristicTokenCounter {
    pub fn new() -> crate::Result<Self> {
        Ok(Self)
    }
}

impl TokenCounterPort for HeuristicTokenCounter {
    fn count(&self, text: &str) -> TokenCount {
        TokenCount::new(super::shared::count_tokens(text))
    }
}
