use crate::domain::{
    code::{service::parser::Parser as ParserTrait, value::symbol::Symbol},
    common::language::Language,
};

pub struct ParserService<P: ParserTrait> {
    parser: P,
}

impl<P: ParserTrait> ParserService<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn parse_symbols(
        &mut self,
        code: &str,
        language: Language,
    ) -> crate::Result<Vec<Symbol>> {
        self.parser.parse_symbols(code, language)
    }
}
