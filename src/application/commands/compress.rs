use crate::application::services::{CompressorService, ParserService};
use crate::domain::code::service::compressor::{
    CompressionLevel, CompressionResult, Compressor as CompressTrait,
};
use crate::domain::code::service::parser::Parser as ParserTrait;
use crate::domain::common::language::Language;
use crate::domain::common::token::TokenBudget;

pub struct CompressCommand {
    pub code: String,
    pub language: Language,
    pub level: CompressionLevel,
    pub budget: Option<u32>,
}

pub fn handle_compress<P, C>(
    cmd: CompressCommand,
    parser: &mut ParserService<P>,
    compressor: &CompressorService<C>,
) -> crate::Result<CompressionResult>
where
    P: ParserTrait,
    C: CompressTrait,
{
    let symbols = parser.parse_symbols(&cmd.code, cmd.language)?;
    let budget = cmd.budget.map(|b| TokenBudget::new(b)).transpose()?;

    let compression_result =
        compressor.compress(&cmd.code, &symbols, cmd.level, budget)?;

    Ok(CompressionResult {
        compressed: compression_result.compressed,
        original_tokens: compression_result.original_tokens,
        compressed_tokens: compression_result.compressed_tokens,
        reduction_percent: compression_result.reduction_percent,
    })
}
