use crate::domain::common::language::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Import,
    Constant,
    TypeAlias,
    Interface,
    Class,
}

#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: u32,
    pub end_line: u32,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub signature: String, // eg: "fn name(arg: Type) -> RetType"
    pub doc: Option<String>, // Doc comment
    pub location: SourceSpan,
    pub language: Language,
    pub dependencies: Vec<String>,
}

impl Symbol {
    pub fn new(
        name: String,
        kind: SymbolKind,
        signature: String,
        language: Language,
        location: SourceSpan,
    ) -> Self {
        Self {
            name,
            kind,
            signature,
            doc: None,
            location,
            language,
            dependencies: Vec::new(),
        }
    }
}
