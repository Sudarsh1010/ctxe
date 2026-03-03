//! Parser trait (domain boundary - no tree-sitter imports!)

use crate::{
    Result,
    domain::{code::value::symbol::Symbol, common::language::Language},
};

/// Domain trait for code parsing
/// Infrastructure adapters implemnts this
pub trait Parser: Send + Sync {
    /// Parse code and extract symbols
    fn parse_symbols(
        &mut self,
        code: &str,
        language: Language,
    ) -> Result<Vec<Symbol>>;

    /// Get the raw AST node text for a symbol (for compression)
    fn extract_signature(
        &mut self,
        code: &str,
        symbol: &Symbol,
    ) -> Result<String>;
}
