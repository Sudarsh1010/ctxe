use crate::models::SymbolKind;
use crate::utils::RowExt;

use rusqlite::Row;
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

impl TryFrom<&Row<'_>> for Symbol {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        let kind = row
            .get::<_, String>(3)?
            .as_str()
            .parse::<SymbolKind>()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    3,
                    "invalid SymbolKind".into(),
                    rusqlite::types::Type::Text,
                )
            })?;

        Ok(Symbol {
            id: row.get(0)?,
            file_id: row.get(1)?,
            name: row.get(2)?,
            kind,
            signature: row.get(4)?,
            body: row.get(5)?,
            start_line: row.get_usize(6)?,
            end_line: row.get_usize(7)?,
        })
    }
}
