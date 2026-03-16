# Workspace/Monorepo Conversion Plan

## TL;DR

> **Quick Summary**: Convert the ctxe Rust project from a single crate to a 5-crate workspace structure using `crates/*` organization. Pure structural refactoring with zero behavior changes.
> 
> **Deliverables**:
> - Virtual workspace root `Cargo.toml`
> - 5 member crates: `ctxe-core`, `ctxe-store`, `ctxe-indexer`, `ctxe`, `ctxe-mcp`
> - Preserved git history and public API compatibility
>
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 4 waves with max 5 parallel tasks
> **Critical Path**: Task 1 → Tasks 2-6 → Task 7 → Task 10 → Task 11 → Task 12 → Task 13

---

## Context

### Original Request
Convert the existing Rust project (ctxe) to a workspace/monorepo structure with `crates/*` organization. No new features needed - just structural reorganization using idiomatic Rust patterns.

### Interview Summary
**Key Discussions**:
- **Granularity**: 5 crates chosen (ctxe, ctxe-core, ctxe-store, ctxe-indexer, ctxe-mcp)
- **Tree-sitter**: Keep all 24 languages bundled, no feature flags
- **Edition**: Using edition 2024 (Rust 1.93.1)

**Research Findings**:
- Virtual workspace with `resolver = "2"` is idiomatic
- `workspace.dependencies` eliminates version drift
- `workspace.package` enables metadata inheritance
- Patterns from tokio/serde/tower: stable traits separate from implementations

### Metis Review
**Critical Issues Identified**:
- `DbError` depends on `rusqlite` + `tokio` → Move to `ctxe-store`
- `db_ext.rs` is rusqlite-specific → Move to `ctxe-store`

**Guardrails Applied**:
- No behavior changes while moving files
- Preserve git history with `git mv`
- Keep exact same public API in `ctxe` crate
- No new dependencies
- No "improvements" or refactoring during the move

---

## Work Objectives

### Core Objective
Reorganize the existing single-crate project into a 5-crate workspace without changing any functionality or public API.

### Concrete Deliverables
- Root `Cargo.toml` as virtual workspace (no `[package]` section)
- `crates/ctxe-core/` - models only (Language, Symbol, SymbolKind, File, Relation)
- `crates/ctxe-store/` - store + error + db_ext
- `crates/ctxe-indexer/` - indexer with 24 tree-sitter languages
- `crates/ctxe/` - facade that re-exports all public APIs
- `crates/ctxe-mcp/` - MCP server binary

### Definition of Done
- [x] `cargo build --workspace` succeeds
- [x] `cargo test --workspace` passes all tests
- [x] `cargo clippy --workspace -- -D warnings` passes
- [x] `cargo fmt --all -- --check` passes
- [x] Original `ctxe` crate public API preserved

### Must Have
- All 30 tests pass after conversion
- Git history preserved (files moved, not deleted+created)
- Zero behavior changes

### Must NOT Have (Guardrails)
- No new features or improvements
- No API changes to the `ctxe` crate
- No new dependencies
- No code refactoring "while we're here"
- No fixing clippy warnings during the move
- No adding documentation beyond what exists

---

## Verification Strategy (MANDATORY)

### Test Decision
- **Infrastructure exists**: NO (only unit tests in source files)
- **Automated tests**: None required (structural change only)
- **Framework**: N/A

### QA Policy
Every task includes agent-executed QA scenarios using Bash for cargo commands.

- **Build verification**: `cargo build -p <crate>` or `cargo build --workspace`
- **Test verification**: `cargo test -p <crate>` or `cargo test --workspace`
- **Lint verification**: `cargo clippy --workspace -- -D warnings`
- **Format verification**: `cargo fmt --all -- --check`

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately - workspace setup, 6 tasks, MAX PARALLEL):
├── Task 1: Create workspace Cargo.toml [quick]
├── Task 2: Create ctxe-core skeleton [quick]
├── Task 3: Create ctxe-store skeleton [quick]
├── Task 4: Create ctxe-indexer skeleton [quick]
├── Task 5: Create ctxe skeleton [quick]
└── Task 6: Create ctxe-mcp skeleton [quick]

Wave 2 (After Wave 1 - file migration, 3 tasks, PARALLEL):
├── Task 7: Migrate models to ctxe-core [quick]
├── Task 8: Migrate store + error + db_ext to ctxe-store [quick]
└── Task 9: Migrate indexer to ctxe-indexer [quick]

Wave 3 (After Wave 2 - facade & binary, 2 tasks, SEQUENTIAL):
├── Task 10: Create ctxe facade crate (depends: 5,7,8,9) [quick]
└── Task 11: Migrate binary to ctxe-mcp (depends: 6,10) [quick]

Wave 4 (After Wave 3 - cleanup & verification, 3 tasks, SEQUENTIAL):
├── Task 12: Remove old src/ directory (depends: 11) [quick]
├── Task 13: Full workspace verification (depends: 12) [quick]
└── Task 14: Create atomic commit (depends: 13) [git]

Critical Path: T1 → T2-6 → T7 → T10 → T11 → T12 → T13 → T14
Parallel Speedup: ~50% faster than sequential
Max Concurrent: 6 (Wave 1)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|------------|--------|
| 1 | — | 2,3,4,5,6 |
| 2 | 1 | 7 |
| 3 | 1 | 8 |
| 4 | 1 | 9 |
| 5 | 1 | 10 |
| 6 | 1 | 11 |
| 7 | 2 | 10 |
| 8 | 3,7 | 10 |
| 9 | 4,7 | 10 |
| 10 | 5,7,8,9 | 11 |
| 11 | 6,10 | 12 |
| 12 | 11 | 13 |
| 13 | 12 | 14 |
| 14 | 13 | — |

### Agent Dispatch Summary

- **Wave 1**: **6** quick agents — T1-T6 all `quick`
- **Wave 2**: **3** quick agents — T7-T9 all `quick` with `git-master` skill
- **Wave 3**: **2** quick agents — T10-T11, T11 has `git-master` skill
- **Wave 4**: **3** agents — T12 `git-master`, T13 `quick`, T14 `git`

---

## TODOs

- [x] 1. Create workspace Cargo.toml

  **What to do**:
  - Create root `Cargo.toml` as virtual workspace (no `[package]` section)
  - Add `[workspace]` with `resolver = "2"` and `members = ["crates/*"]`
  - Add `[workspace.package]` section for shared metadata (version, edition, license, etc.)
  - Add `[workspace.dependencies]` section with all current dependencies centralized
  - Add `[workspace.lints]` section for shared linting rules

  **Must NOT do**:
  - Do NOT add a `[package]` section (virtual workspace)
  - Do NOT change any dependency versions
  - Do NOT add dependencies that weren't in the original Cargo.toml

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple file creation with well-defined structure
  - **Skills**: []
    - No special skills needed for this task

  **Parallelization**:
  - **Can Run In Parallel**: NO (foundation task)
  - **Parallel Group**: Wave 1 (first task)
  - **Blocks**: Tasks 2, 3, 4, 5, 6
  - **Blocked By**: None

  **References**:
  - Current `Cargo.toml` at project root - copy all dependencies exactly
  - `rust-toolchain.toml` - edition is 2024, channel is 1.93.1
  - Research findings on workspace structure from tokio/serde patterns

  **Acceptance Criteria**:
  - [ ] Root Cargo.toml has `[workspace]` section with `resolver = "2"`
  - [ ] `members = ["crates/*"]` is set
  - [ ] `[workspace.package]` has version, edition = "2024", license = "MIT"
  - [ ] `[workspace.dependencies]` contains all original dependencies
  - [ ] `cargo metadata --format-version 1` succeeds

  **QA Scenarios**:
  ```
  Scenario: Workspace structure is valid
    Tool: Bash
    Steps:
      1. cargo metadata --format-version 1 | jq '.workspace_members'
    Expected Result: Returns valid JSON (empty array is OK, no crates yet)
    Failure Indicators: Error parsing Cargo.toml
    Evidence: .sisyphus/evidence/task-01-workspace-valid.txt

  Scenario: All dependencies captured in workspace
    Tool: Bash
    Steps:
      1. grep -E "^\[workspace.dependencies\]" Cargo.toml
      2. grep "rmcp\|tokio\|serde\|rusqlite\|tree-sitter" Cargo.toml | wc -l
    Expected Result: Section exists, major deps present
    Failure Indicators: Missing [workspace.dependencies] section
    Evidence: .sisyphus/evidence/task-01-deps-present.txt
  ```

  **Commit**: NO (wait for final commit)

- [x] 2. Create ctxe-core crate skeleton

  **What to do**:
  - Run `cargo new --lib crates/ctxe-core`
  - Edit `crates/ctxe-core/Cargo.toml` to inherit from workspace:
    - `version.workspace = true`
    - `edition.workspace = true`
    - `license.workspace = true`
    - `name = "ctxe-core"`
  - Add description: `description = "Core data models for ctxe"`
  - Leave `src/lib.rs` as empty `pub fn placeholder() {}` for now

  **Must NOT do**:
  - Do NOT add any dependencies yet
  - Do NOT copy any source files yet

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple cargo new + config edits
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 3, 4, 5, 6 after Task 1 completes)
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 7
  - **Blocked By**: Task 1

  **References**:
  - Root `Cargo.toml` workspace.package section for inheritance

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-core/` directory exists
  - [ ] `crates/ctxe-core/Cargo.toml` has workspace inheritance
  - [ ] `cargo build -p ctxe-core` succeeds

  **QA Scenarios**:
  ```
  Scenario: Crate builds successfully
    Tool: Bash
    Steps:
      1. cargo build -p ctxe-core
    Expected Result: "Compiling ctxe-core v..." then success
    Failure Indicators: Compilation errors
    Evidence: .sisyphus/evidence/task-02-core-builds.txt
  ```

  **Commit**: NO

- [x] 3. Create ctxe-store crate skeleton

  **What to do**:
  - Run `cargo new --lib crates/ctxe-store`
  - Edit `crates/ctxe-store/Cargo.toml` to inherit from workspace
  - Add description: `description = "Database storage layer for ctxe"`
  - Leave `src/lib.rs` as placeholder

  **Must NOT do**:
  - Do NOT add dependencies yet

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 2, 4, 5, 6)
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 8
  - **Blocked By**: Task 1

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-store/` directory exists
  - [ ] `cargo build -p ctxe-store` succeeds

  **QA Scenarios**:
  ```
  Scenario: Crate builds successfully
    Tool: Bash
    Steps:
      1. cargo build -p ctxe-store
    Expected Result: Compilation succeeds
    Evidence: .sisyphus/evidence/task-03-store-builds.txt
  ```

  **Commit**: NO

- [x] 4. Create ctxe-indexer crate skeleton

  **What to do**:
  - Run `cargo new --lib crates/ctxe-indexer`
  - Edit `crates/ctxe-indexer/Cargo.toml` to inherit from workspace
  - Add description: `description = "Code parsing and indexing with tree-sitter"`
  - Leave `src/lib.rs` as placeholder

  **Must NOT do**:
  - Do NOT add dependencies yet

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 2, 3, 5, 6)
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 9
  - **Blocked By**: Task 1

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-indexer/` directory exists
  - [ ] `cargo build -p ctxe-indexer` succeeds

  **QA Scenarios**:
  ```
  Scenario: Crate builds successfully
    Tool: Bash
    Steps:
      1. cargo build -p ctxe-indexer
    Expected Result: Compilation succeeds
    Evidence: .sisyphus/evidence/task-04-indexer-builds.txt
  ```

  **Commit**: NO

- [x] 5. Create ctxe crate skeleton

  **Commit**: NO

- [x] 6. Create ctxe-mcp binary crate skeleton

  **What to do**:
  - Run `cargo new --bin crates/ctxe-mcp`
  - Edit `crates/ctxe-mcp/Cargo.toml` to inherit from workspace
  - Add description: `description = "MCP server binary for ctxe"`
  - Leave `src/main.rs` as placeholder `fn main() {}`

  **Must NOT do**:
  - Do NOT add dependencies yet

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 2, 3, 4, 5)
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 11
  - **Blocked By**: Task 1

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-mcp/` directory exists
  - [ ] `cargo build -p ctxe-mcp` succeeds

  **QA Scenarios**:
  ```
  Scenario: Binary crate builds successfully
    Tool: Bash
    Steps:
      1. cargo build -p ctxe-mcp
    Expected Result: Compilation succeeds
    Evidence: .sisyphus/evidence/task-06-mcp-builds.txt
  ```

  **Commit**: NO

- [x] 7. Migrate models to ctxe-core

  **What to do**:
  - Move models: `git mv src/models crates/ctxe-core/src/`
  - Add dependencies to `crates/ctxe-core/Cargo.toml`:
    - `serde = { workspace = true }`
    - `thiserror = { workspace = true }` (if models need it)
  - Update `crates/ctxe-core/src/lib.rs`:
    ```rust
    pub mod models;
    pub use models::*;
    ```
  - Fix any internal imports from `crate::models::` to `crate::models::` (should work as-is)

  **Must NOT do**:
  - Do NOT change any code logic
  - Do NOT add new derives or traits
  - Do NOT fix clippy warnings

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: File move + simple config edits
  - **Skills**: [`git-master`]
    - `git-master`: Use `git mv` to preserve history

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 8, 9 after Task 2 completes)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 10
  - **Blocked By**: Task 2

  **References**:
  - `src/models/mod.rs` - structure to replicate in lib.rs
  - `src/models/language.rs` - has 27 tests that must pass

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-core/src/models/` exists with all model files
  - [ ] `crates/ctxe-core/src/lib.rs` exports models module
  - [ ] `cargo test -p ctxe-core` passes all tests

  **QA Scenarios**:
  ```
  Scenario: Tests pass in ctxe-core
    Tool: Bash
    Steps:
      1. cargo test -p ctxe-core
    Expected Result: All tests pass (27 tests in language.rs)
    Failure Indicators: Any test failures
    Evidence: .sisyphus/evidence/task-07-core-tests.txt

  Scenario: Git history preserved
    Tool: Bash
    Steps:
      1. git log --follow --oneline crates/ctxe-core/src/models/language.rs | head -5
    Expected Result: Shows commits dating back to original file creation
    Failure Indicators: Only shows new commits (history lost)
    Evidence: .sisyphus/evidence/task-07-git-history.txt
  ```

  **Commit**: NO

- [x] 8. Migrate store + error + db_ext to ctxe-store

  **What to do**:
  - Move store: `git mv src/store crates/ctxe-store/src/`
  - Move error: `git mv src/error.rs crates/ctxe-store/src/`
  - Move db_ext: `git mv src/utils/db_ext.rs crates/ctxe-store/src/`
  - Add dependencies to `crates/ctxe-store/Cargo.toml`:
    - `ctxe-core = { path = "../ctxe-core" }`
    - `rusqlite = { workspace = true }`
    - `sqlite-vec = { workspace = true }`
    - `tokio = { workspace = true }`
    - `dirs = { workspace = true }`
    - `thiserror = { workspace = true }`
  - Update `crates/ctxe-store/src/lib.rs`:
    ```rust
    pub mod store;
    pub mod error;
    pub mod db_ext;
    pub use store::*;
    pub use error::*;
    pub use db_ext::*;
    ```
  - Fix imports in store files:
    - Change `crate::error::DbError` imports (should work with new location)
    - Change `crate::models::*` to `ctxe_core::*` where needed
    - Change `crate::utils::RowExt` to `crate::db_ext::RowExt`

  **Must NOT do**:
  - Do NOT change error type definitions
  - Do NOT refactor database code

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 7, 9)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 10
  - **Blocked By**: Task 3, Task 7 (needs ctxe-core)

  **References**:
  - `src/store/mod.rs` - uses `crate::error::DbError`
  - `src/models/symbol.rs` - uses `crate::utils::RowExt` (but symbol is in core, so this needs updating)

  **Critical Note**: `symbol.rs` in ctxe-core uses `RowExt` from utils. This creates a dependency from core to store which is backwards. Solution: Keep `RowExt` trait in store, and update symbol.rs to NOT implement `TryFrom<&Row>` - move that impl to store crate.

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-store/src/store/` exists
  - [ ] `crates/ctxe-store/src/error.rs` exists
  - [ ] `crates/ctxe-store/src/db_ext.rs` exists
  - [ ] `cargo build -p ctxe-store` succeeds

  **QA Scenarios**:
  ```
  Scenario: Store crate builds
    Tool: Bash
    Steps:
      1. cargo build -p ctxe-store
    Expected Result: Compilation succeeds
    Failure Indicators: Import errors, missing deps
    Evidence: .sisyphus/evidence/task-08-store-migrated.txt
  ```

  **Commit**: NO

- [x] 9. Migrate indexer to ctxe-indexer

  **What to do**:
  - Move indexer: `git mv src/indexer crates/ctxe-indexer/src/`
  - Add dependencies to `crates/ctxe-indexer/Cargo.toml`:
    - `ctxe-core = { path = "../ctxe-core" }`
    - `tree-sitter = { workspace = true }`
    - All 24 tree-sitter language deps from workspace
    - `ignore = { workspace = true }`
    - `sha2 = { workspace = true }`
    - `hex = { workspace = true }`
    - `rayon = { workspace = true }`
    - `thiserror = { workspace = true }`
  - Update `crates/ctxe-indexer/src/lib.rs`:
    ```rust
    pub mod indexer;
    pub use indexer::*;
    ```
  - Fix imports in indexer files:
    - Change `crate::models::SymbolKind` to `ctxe_core::SymbolKind`

  **Must NOT do**:
  - Do NOT change language definitions
  - Do NOT remove any tree-sitter languages

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 7, 8)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 10
  - **Blocked By**: Task 4, Task 7 (needs ctxe-core with SymbolKind)

  **References**:
  - `src/indexer/languages.rs` - imports `crate::models::SymbolKind`
  - `src/indexer/hash.rs` - has 3 tests that must pass

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-indexer/src/indexer/` exists with all files
  - [ ] All 24 tree-sitter languages still included
  - [ ] `cargo test -p ctxe-indexer` passes

  **QA Scenarios**:
  ```
  Scenario: Indexer tests pass
    Tool: Bash
    Steps:
      1. cargo test -p ctxe-indexer
    Expected Result: All tests pass (3 tests in hash.rs)
    Evidence: .sisyphus/evidence/task-09-indexer-tests.txt

  Scenario: All languages still included
    Tool: Bash
    Steps:
      1. grep -c "tree-sitter-" crates/ctxe-indexer/Cargo.toml
    Expected Result: Count is 24 or more
    Failure Indicators: Missing language dependencies
    Evidence: .sisyphus/evidence/task-09-languages-present.txt
  ```

  **Commit**: NO

- [x] 10. Create ctxe facade crate

  **What to do**:
  - Add dependencies to `crates/ctxe/Cargo.toml`:
    - `ctxe-core = { path = "../ctxe-core" }`
    - `ctxe-store = { path = "../ctxe-store" }`
    - `ctxe-indexer = { path = "../ctxe-indexer" }`
  - Update `crates/ctxe/src/lib.rs` to re-export everything:
    ```rust
    pub use ctxe_core::*;
    pub use ctxe_store::*;
    pub use ctxe_indexer::*;
    ```
  - Ensure this matches the original `src/lib.rs` API:
    ```rust
    // Original lib.rs exported:
    pub mod error;
    pub mod indexer;
    pub mod models;
    pub mod store;
    pub mod utils;
    ```

  **Must NOT do**:
  - Do NOT add new public items
  - Do NOT change visibility of any items

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO (needs all Wave 2 tasks complete)
  - **Parallel Group**: Wave 3 (first task)
  - **Blocks**: Task 11
  - **Blocked By**: Task 5, Task 7, Task 8, Task 9

  **References**:
  - `src/lib.rs` - original exports to match

  **Acceptance Criteria**:
  - [ ] `crates/ctxe/src/lib.rs` re-exports all public items
  - [ ] `cargo build -p ctxe` succeeds
  - [ ] `use ctxe::{models::*, store::*, indexer::*, error::*, utils::*};` would compile

  **QA Scenarios**:
  ```
  Scenario: Facade builds and exports correctly
    Tool: Bash
    Steps:
      1. cargo build -p ctxe
    Expected Result: Compilation succeeds
    Evidence: .sisyphus/evidence/task-10-facade-builds.txt
  ```

  **Commit**: NO

- [x] 11. Migrate binary to ctxe-mcp

  **What to do**:
  - Move binary: `git mv src/bin/ctxe-mcp/main.rs crates/ctxe-mcp/src/main.rs`
  - Add dependencies to `crates/ctxe-mcp/Cargo.toml`:
    - `ctxe = { path = "../ctxe" }`
    - `rmcp = { workspace = true }`
    - `tokio = { workspace = true }`
  - The binary currently only uses `rmcp` and `tokio`, not the ctxe library yet

  **Must NOT do**:
  - Do NOT change binary functionality
  - Do NOT add new features to the MCP server

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Parallelization**:
  - **Can Run In Parallel**: NO (needs Task 10 complete)
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 12
  - **Blocked By**: Task 6, Task 10

  **References**:
  - `src/bin/ctxe-mcp/main.rs` - current binary code

  **Acceptance Criteria**:
  - [ ] `crates/ctxe-mcp/src/main.rs` exists
  - [ ] `cargo build -p ctxe-mcp` succeeds
  - [ ] Binary runs without error

  **QA Scenarios**:
  ```
  Scenario: Binary builds and runs
    Tool: Bash
    Steps:
      1. cargo build -p ctxe-mcp --release
      2. timeout 2 ./target/release/ctxe-mcp || true
    Expected Result: Build succeeds, binary starts (may timeout waiting for stdin)
    Failure Indicators: Compilation errors, immediate crash
    Evidence: .sisyphus/evidence/task-11-binary-works.txt
  ```

  **Commit**: NO

- [x] 12. Remove old src/ directory

  **What to do**:
  - Verify all code has been migrated: `ls src/` should only have empty dirs or already-moved files
  - Remove old structure: `git rm -r src/`
  - Verify root `Cargo.toml` is still a virtual workspace (no `[package]` section leaked in)

  **Must NOT do**:
  - Do NOT remove any files that haven't been migrated
  - Do NOT modify Cargo.lock

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 13
  - **Blocked By**: Task 11

  **References**:
  - Original `src/` directory structure

  **Acceptance Criteria**:
  - [ ] `src/` directory no longer exists
  - [ ] Root `Cargo.toml` is virtual workspace only
  - [ ] `cargo build --workspace` still succeeds

  **QA Scenarios**:
  ```
  Scenario: Old src removed, workspace still works
    Tool: Bash
    Steps:
      1. ls src/ 2>&1 || echo "src/ removed"
      2. cargo build --workspace
    Expected Result: src/ not found, workspace builds
    Evidence: .sisyphus/evidence/task-12-cleanup.txt
  ```

  **Commit**: NO

- [x] 13. Full workspace verification

  **What to do**:
  - Run all tests: `cargo test --workspace`
  - Run clippy: `cargo clippy --workspace --all-targets -- -D warnings`
  - Run fmt check: `cargo fmt --all -- --check`
  - Verify build: `cargo build --workspace --all-targets`
  - Check that all 30+ tests pass

  **Must NOT do**:
  - Do NOT fix warnings by changing code logic
  - Do NOT add new tests

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 14
  - **Blocked By**: Task 12

  **References**:
  - Original test counts: 27 in language.rs, 3 in hash.rs

  **Acceptance Criteria**:
  - [ ] `cargo test --workspace` passes all tests
  - [ ] `cargo clippy --workspace -- -D warnings` passes
  - [ ] `cargo fmt --all -- --check` passes
  - [ ] No compilation errors

  **QA Scenarios**:
  ```
  Scenario: All tests pass
    Tool: Bash
    Steps:
      1. cargo test --workspace 2>&1 | tee /tmp/test-output.txt
      2. grep -E "test result: ok" /tmp/test-output.txt
    Expected Result: All tests pass, "ok" shown for each crate
    Failure Indicators: Any test failures
    Evidence: .sisyphus/evidence/task-13-tests.txt

  Scenario: Clippy passes
    Tool: Bash
    Steps:
      1. cargo clippy --workspace --all-targets -- -D warnings 2>&1
    Expected Result: No warnings, exit code 0
    Failure Indicators: Clippy warnings or errors
    Evidence: .sisyphus/evidence/task-13-clippy.txt

  Scenario: Format is correct
    Tool: Bash
    Steps:
      1. cargo fmt --all -- --check
    Expected Result: No formatting changes needed
    Failure Indicators: "Diff in" messages
    Evidence: .sisyphus/evidence/task-13-fmt.txt
  ```

  **Commit**: NO

- [x] 14. Create atomic commit

  **What to do**:
  - Stage all changes: `git add -A`
  - Create single commit: `git commit -m "refactor: convert to workspace structure with 5 crates"`
  - Verify commit shows file moves: `git show --stat HEAD`
  - Verify history preservation: `git log --follow crates/ctxe-core/src/models/language.rs`

  **Must NOT do**:
  - Do NOT create multiple commits
  - Do NOT include unrelated changes

  **Recommended Agent Profile**:
  - **Category**: `git`
    - Reason: Git operations with history preservation
  - **Skills**: [`git-master`]
    - `git-master`: Ensure atomic commit with proper message

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (final)
  - **Blocks**: None
  - **Blocked By**: Task 13

  **References**:
  - Git history from original files

  **Acceptance Criteria**:
  - [ ] Single commit with all changes
  - [ ] Commit message follows conventional commits
  - [ ] `git show --stat` shows reasonable changes (moves, not mass deletions)
  - [ ] Git history preserved for key files

  **QA Scenarios**:
  ```
  Scenario: Commit is atomic and correct
    Tool: Bash
    Steps:
      1. git log --oneline -1
      2. git show --stat HEAD | head -30
    Expected Result: Single commit shown, files listed as renamed/moved
    Evidence: .sisyphus/evidence/task-14-commit.txt

  Scenario: Git history preserved
    Tool: Bash
    Steps:
      1. git log --follow --oneline crates/ctxe-core/src/models/language.rs | head -5
    Expected Result: Shows commits from original file location
    Failure Indicators: Only shows new commit
    Evidence: .sisyphus/evidence/task-14-history.txt
  ```

  **Commit**: YES (this IS the commit)
  - Message: `refactor: convert to workspace structure with 5 crates`
  - Pre-commit: `cargo test --workspace && cargo clippy --workspace -- -D warnings`

---

## Final Verification Wave (MANDATORY)

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. Verify: workspace Cargo.toml is virtual (no [package]), 5 member crates exist, all files moved from src/, tests pass, no new dependencies in Cargo.lock.
  Output: `Workspace [YES/NO] | Crates [5/5] | Files Moved [N/N] | Tests [PASS/FAIL] | VERDICT`

- [x] F2. **Code Quality Review** — `unspecified-high`
   Run `cargo clippy --workspace -- -D warnings` + `cargo fmt --all -- --check`. Check for any unintended code changes by comparing git diff statistics (should only show file moves, not content changes except import paths).
   Output: `Clippy [PASS/FAIL] | Format [PASS/FAIL] | Changes [MOVED-ONLY/MODIFIED] | VERDICT`
   - [ ] F3. **API Compatibility Check** — `unspecified-high`
   Verify that `use ctxe::{models::*, store::*, indexer::*, error::*, utils::*};` compiles. Check that all public items from original lib.rs are accessible through the new ctxe crate.
   Output: `Imports [PASS/FAIL] | Re-exports [N/N] | VERDICT`

- [ ] F3. **API Compatibility Check** — `unspecified-high`
  Verify that `use ctxe::{models::*, store::*, indexer::*, error::*, utils::*};` compiles. Check that all public items from original lib.rs are accessible through the new ctxe crate.
  Output: `Imports [PASS/FAIL] | Re-exports [N/N] | VERDICT`

- [x] F3. **API Compatibility Check** — `unspecified-high`
   Verify that `use ctxe::{models::*, store::*, indexer::*, error::*, utils::*};` compiles. Check that all public items from original lib.rs are accessible through the new ctxe crate.
   Output: `Imports [PASS/FAIL] | Re-exports [N/N] | VERDICT`

---

## Commit Strategy

- **Single Commit**: All changes in one atomic commit
  - Message: `refactor: convert to workspace structure with 5 crates`
  - Pre-commit: `cargo test --workspace && cargo clippy --workspace -- -D warnings`
  - Use `git add -A` after all tasks complete

---

## Success Criteria

### Verification Commands
```bash
cargo build --workspace              # Expected: success
cargo test --workspace               # Expected: all tests pass
cargo clippy --workspace -- -D warnings  # Expected: no warnings
cargo fmt --all -- --check           # Expected: no changes needed
cargo run -p ctxe-mcp                # Expected: binary starts
```

### Final Checklist
- [x] Workspace root has virtual Cargo.toml (no [package])
- [x] 5 member crates in crates/* directory
- [x] All tests pass (30+ tests)
- [x] No new dependencies added
- [ ] Git history preserved
- [x] ctxe crate API unchanged
