# Learnings

## Hash Function Implementation Pattern

The merkle hash function follows the existing pattern in `src/store/path.rs`:
- Uses `sha2::{Digest, Sha256}` for hashing
- Uses `hex::encode()` to produce hex string output
- Uses `std::path::Path` as input type

## Memory Efficiency

Implemented chunked file reading (8KB buffer) to handle large files without loading entire content into memory. This is important for performance when indexing large codebases.

## Error Handling

Used `thiserror::Error` with `#[from]` derive to convert `std::io::Error` into `HashError` automatically.

## Testing Strategy

- Used `tempfile::NamedTempFile` for isolated test file creation
- Tests cover edge cases: empty file, small text file, and large file (1MB)
- All tests pass with correct SHA-256 hash values

## Module Integration

Successfully integrated hash module into `src/indexer/mod.rs`:
- Added `mod hash;` declaration
- Added `pub use hash::*;` to export the public API
