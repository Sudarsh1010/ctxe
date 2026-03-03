//! Compressor service (orchestrates domain + infrastructure)

use crate::Result;
use crate::domain::code::service::compressor::{
    CompressionLevel, CompressionResult, Compressor as CompressorTrait,
};
use crate::domain::code::value::symbol::Symbol;
use crate::domain::common::token::TokenBudget;

pub struct CompressorService<C: CompressorTrait> {
    compressor: C,
}

impl<C: CompressorTrait> CompressorService<C> {
    pub fn new(compressor: C) -> Self {
        Self { compressor }
    }

    pub fn compress(
        &self,
        code: &str,
        symbols: &[Symbol],
        level: CompressionLevel,
        budget: Option<TokenBudget>,
    ) -> Result<CompressionResult> {
        self.compressor.compress(code, symbols, level, budget)
    }
}
