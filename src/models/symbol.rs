use serde::{Deserialize, Serialize};

use crate::models::SymbolKind;

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
