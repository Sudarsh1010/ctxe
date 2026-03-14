use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Import,
    Constant,
    TypeAlias,
    Interface,
    Module,
    Class,
    Method,
    Variable,
}

impl SymbolKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SymbolKind::Function => "function",
            SymbolKind::Struct => "struct",
            SymbolKind::Enum => "enum",
            SymbolKind::Trait => "trait",
            SymbolKind::Module => "module",
            SymbolKind::Class => "class",
            SymbolKind::Method => "method",
            SymbolKind::Variable => "variable",
            SymbolKind::Impl => "impl",
            SymbolKind::Import => "import",
            SymbolKind::Constant => "constant",
            SymbolKind::TypeAlias => "type_alias",
            SymbolKind::Interface => "interface",
        }
    }
}

impl FromStr for SymbolKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "function" => Ok(Self::Function),
            "struct" => Ok(Self::Struct),
            "enum" => Ok(Self::Enum),
            "trait" => Ok(Self::Trait),
            "module" => Ok(Self::Module),
            "class" => Ok(Self::Class),
            "method" => Ok(Self::Method),
            "variable" => Ok(Self::Variable),
            "impl" => Ok(Self::Impl),
            "import" => Ok(Self::Import),
            "constant" => Ok(Self::Constant),
            "type_alias" => Ok(Self::TypeAlias),
            "interface" => Ok(Self::Interface),
            _ => Err("invalid SymbolKind value"),
        }
    }
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
