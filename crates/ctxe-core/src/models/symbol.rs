use crate::models::SymbolKind;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: i64,
    pub file_id: i64,
    pub signature: Option<String>,
    pub name: String,
    pub kind: SymbolKind,
    pub body: Option<String>,
    pub start_line: usize,
    pub end_line: usize,
}

// Note: TryFrom<&Row> implementation moved to ctxe-store to avoid rusqlite dependency in core
