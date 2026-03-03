//! Compressor trait (domain boundary)

use crate::domain::{code::value::symbol::Symbol, common::token::TokenBudget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// Signatures Only (~70% reduction)
    SignaturesOnly,

    /// Signatures + Docs (~60% reduction)
    WithDocs,

    /// Full code minus comments/whitespace (~30% reduction)
    RemoveComments,

    None,
}

pub trait Compressor: Send + Sync {
    /// Compress code to fit within token budget
    fn compress(
        &self,
        code: &str,
        symbols: &[Symbol],
        level: CompressionLevel,
        budget: Option<TokenBudget>,
    ) -> crate::Result<CompressionResult>;
}

#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub compressed: String,
    pub original_tokens: u32,
    pub compressed_tokens: u32,
    pub reduction_percent: f64,
}

impl CompressionResult {
    pub fn new(
        compressed: String,
        original: u32,
        compressed_count: u32,
    ) -> Self {
        let reduction = if original > 0 {
            100.0 * (1.0 - (compressed_count as f64 / original as f64))
        } else {
            0.0
        };

        Self {
            compressed,
            original_tokens: original,
            compressed_tokens: compressed_count,
            reduction_percent: reduction,
        }
    }
}
