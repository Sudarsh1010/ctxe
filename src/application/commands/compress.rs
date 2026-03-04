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

impl CompressCommand {
    pub fn new(
        code: String,
        language: Language,
        level: CompressionLevel,
        budget: Option<u32>,
    ) -> crate::Result<Self> {
        if code.is_empty() {
            return Err(crate::Error::Parse("Code cannot be empty".into()));
        }

        if let Some(b) = budget {
            TokenBudget::new(b)?; // Validate budget
        }

        Ok(Self {
            code,
            language,
            level,
            budget,
        })
    }
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
    let budget = cmd.budget.map(TokenBudget::new).transpose()?;

    let compression_result =
        compressor.compress(&cmd.code, &symbols, cmd.level, budget)?;

    Ok(CompressionResult {
        compressed: compression_result.compressed,
        original_tokens: compression_result.original_tokens,
        compressed_tokens: compression_result.compressed_tokens,
        reduction_percent: compression_result.reduction_percent,
    })
}
