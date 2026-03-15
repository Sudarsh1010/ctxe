# Merkle Hash Implementation

## TL;DR

> **Quick Summary**: Implement SHA-256 hash calculation for file contents to enable change detection.
> 
> **Deliverables**:
> - `src/indexer/hash.rs` - Hash calculation module
> - `compute_merkle_hash(path: &Path) -> Result<String, HashError>`
> - Unit tests
> 
> **Estimated Effort**: Quick
> **Parallel Execution**: NO - single task

---

## Context

### Original Request
Implement Step 2 of plan.md - Merkle hash calculation for change detection.

### Existing Infrastructure
- `sha2` crate in Cargo.toml ✓
- `hex` crate in Cargo.toml ✓
- Hash pattern in `src/store/path.rs:6-8` ✓
- `File` model has `merkle_hash: String` ✓

---

## TODOs

- [x] 1. Create Hash Module

  **What to do**:
  - Create `src/indexer/hash.rs`
  - Implement `compute_merkle_hash(path: &Path) -> Result<String, HashError>`
  - Use `sha2::Sha256` to hash file contents
  - Return hex-encoded string
  - Define `HashError` with `Io` variant

  **References**:
  - `src/store/path.rs:6-8` - Existing hash pattern

  **Acceptance Criteria**:
  - [ ] Function reads file contents
  - [ ] SHA-256 hash computed
  - [ ] Returns hex-encoded string
  - [ ] Unit tests pass

  **QA Scenarios**:
  ```
  Scenario: Hash computation works
    Tool: Bash
    Steps:
      1. Create test file with known content
      2. Call compute_merkle_hash
      3. Verify hash matches expected SHA-256
    Expected Result: Correct hex string
  ```

  **Commit**: YES
  - Message: `feat(indexer): add merkle hash calculation`

---

## Final Verification Wave

- [x] F1. Code compiles with `cargo check`
- [x] F2. Tests pass with `cargo test`
- [x] F3. Clippy clean

---

## Success Criteria

```bash
cargo test                    # All tests pass
cargo clippy -- -D warnings   # No warnings
```
