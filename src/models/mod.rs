mod language;
mod symbol;
mod symbol_kind;

use serde::{Deserialize, Serialize};

pub use language::*;
pub use symbol::*;
pub use symbol_kind::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub path: String,
    pub file_hash: String,
    pub language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub source_symbol_id: i64,
    pub target_symbol_id: Option<i64>,
    pub target_external_name: Option<String>,
    pub relation_type: String, // CALLS, IMPORTS
}
