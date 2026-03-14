/// # Example
/// ```rust
/// use crate::utils::RowExt;
/// let line: usize = row.get_usize(0)?;
/// ```
use rusqlite::{Error, Row, RowIndex};

/// Extension trait for `rusqlite::Row` to support ergonomic fetching of `usize`.
pub trait RowExt {
    /// Fetch a column as `usize`, with overflow checking.
    fn get_usize<I: RowIndex>(&self, idx: I) -> Result<usize, Error>;
}

impl RowExt for Row<'_> {
    fn get_usize<I: RowIndex>(&self, idx: I) -> Result<usize, Error> {
        let val: i64 = self.get(idx)?;
        val.try_into()
            .map_err(|_| Error::IntegralValueOutOfRange(0, val)) // 0 = "unknown column", still informative
    }
}
