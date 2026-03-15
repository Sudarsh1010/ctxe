# Tree-sitter Setup & Gitignore Parsing

## TL;DR

> **Quick Summary**: Implement file discovery module with gitignore/indexignore support for the structural code indexer. This is Phase 1 Step 1 of the plan.md - walk directories, apply ignore rules, return indexable files.
> 
> **Deliverables**:
> - `src/indexer/walker.rs` - File discovery module
> - `src/models/language.rs` - Expanded Language enum (6 → 24 languages)
> - `src/indexer/discovered.rs` - DiscoveredFile struct
> - Test coverage for all components
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 → Task 3 → Task 5

---

## Context

### Original Request
"Setup tree sitter and implement logic to parse gitignore and stuff. Phase 1 step 1 of plan.md"

### Interview Summary
**Key Discussions**:
- **Ignore files**: Support both .gitignore AND .indexignore
- **Deliverable scope**: File discovery only, NOT parsing (that's Step 3)
- **Module location**: New files under `src/indexer/`
- **Language enum**: Expand from 6 to 24 languages to match LANGUAGES array
- **Test strategy**: TDD - write tests first

**Research Findings**:
- **ignore crate** (ripgrep's): Gold standard for gitignore + WalkBuilder
- **rayon crate**: For parallel processing (add to dependencies)
- **Tree-sitter Parser**: NOT thread-safe, create one per thread
- **Tree-sitter Language**: Thread-safe (Send + Sync), can share via Arc

### Metis Review
**Identified Gaps** (addressed):
- **Output format**: Resolved - return `DiscoveredFile` struct with path + language
- **Hidden files**: Resolved - EXCLUDE hidden directories/files
- **Symlinks**: Resolved - DO NOT follow symlinks (safer, prevents cycles)
- **File size**: Resolved - No limit for now
- **Error handling**: Log and continue (don't fail entire walk)

---

## Work Objectives

### Core Objective
Create a file discovery module that walks a project directory, respects gitignore/indexignore rules, and returns a list of indexable files with their detected languages.

### Concrete Deliverables
- `src/indexer/walker.rs` - Main walker module with `discover_files()` function
- `src/indexer/discovered.rs` - `DiscoveredFile` struct definition
- `src/models/language.rs` - Expanded `Language` enum (24 variants)
- `Cargo.toml` - Add `ignore` and `rayon` dependencies
- Test files for TDD coverage

### Definition of Done
- [ ] `cargo test` passes all tests
- [ ] `cargo clippy` passes with no warnings
- [ ] Walker respects .gitignore rules correctly
- [ ] Walker respects .indexignore rules correctly
- [ ] Walker excludes hidden files/directories
- [ ] Walker does not follow symlinks
- [ ] Language enum has all 24 language variants
- [ ] DiscoveredFile struct includes path + language

### Must Have
- Directory walking with ignore rules
- .gitignore parsing support
- .indexignore parsing support (custom ignore file)
- Language detection from file extension
- DiscoveredFile struct with path and language

### Must NOT Have (Guardrails)
- NO file parsing with tree-sitter (that's Step 3)
- NO embedding generation (that's Step 5)
- NO database persistence (that's Step 6)
- NO file watching/hot reload (that's Phase 2)
- NO excessive dependencies beyond `ignore` and `rayon`

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: NO (but will be set up)
- **Automated tests**: YES (TDD)
- **Framework**: `cargo test` (built-in Rust test framework)
- **TDD**: Each task follows RED (failing test) → GREEN (minimal impl) → REFACTOR

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Backend/Library**: Use Bash (cargo test, cargo clippy) — Run tests, lint, verify compilation
- **Integration**: Use Bash (cargo run with test fixtures) — Run against test project

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — foundation + types):
├── Task 1: Add dependencies to Cargo.toml [quick]
├── Task 2: Expand Language enum to 24 variants [quick]
└── Task 3: Define DiscoveredFile struct [quick]

Wave 2 (After Wave 1 — core implementation):
├── Task 4: Implement walker module with ignore rules [deep]
└── Task 5: Integrate language detection with walker [unspecified-high]

Wave 3 (After Wave 2 — verification + cleanup):
├── Task 6: Integration test with fixture project [unspecified-high]
└── Task 7: Update indexer/mod.rs exports [quick]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 4 → Task 5 → Task 6 → F1-F4
Parallel Speedup: ~50% faster than sequential
Max Concurrent: 3 (Wave 1)
```

### Dependency Matrix

- **1**: — — 4
- **2**: — — 5
- **3**: — — 4, 5
- **4**: 1, 3 — 5
- **5**: 2, 4 — 6
- **6**: 5 — 7, F1-F4
- **7**: 6 — F1-F4

### Agent Dispatch Summary

- **Wave 1**: **3** — T1 → `quick`, T2 → `quick`, T3 → `quick`
- **Wave 2**: **2** — T4 → `deep`, T5 → `unspecified-high`
- **Wave 3**: **2** — T6 → `unspecified-high`, T7 → `quick`
- **FINAL**: **4** — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

- [x] 1. Add Dependencies to Cargo.toml

  **What to do**:
  - Add `ignore = "0.4"` dependency for gitignore parsing + WalkBuilder
  - Add `rayon = "1.10"` dependency for parallel processing
  - Verify compilation with `cargo check`

  **Must NOT do**:
  - Do NOT add any other dependencies (avoid scope creep)
  - Do NOT update existing dependencies

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple addition of two dependencies to existing Cargo.toml
  - **Skills**: []
    - No special skills needed for this task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Task 4 (needs ignore crate)
  - **Blocked By**: None (can start immediately)

  **References**:
  - `Cargo.toml:25-51` - Current dependencies section format to follow
  - https://crates.io/crates/ignore - ignore crate version info
  - https://crates.io/crates/rayon - rayon crate version info

  **Acceptance Criteria**:
  - [ ] `ignore = "0.4"` added to [dependencies]
  - [ ] `rayon = "1.10"` added to [dependencies]
  - [ ] `cargo check` passes without errors

  **QA Scenarios**:

  ```
  Scenario: Dependencies compile correctly
    Tool: Bash
    Preconditions: Cargo.toml updated with new dependencies
    Steps:
      1. Run `cargo check`
      2. Verify exit code is 0
    Expected Result: Compilation succeeds with no errors
    Failure Indicators: Compilation errors about missing crates
    Evidence: .sisyphus/evidence/task-1-deps-compile.txt
  ```

  **Evidence to Capture**:
  - [ ] `cargo check` output showing success

  **Commit**: NO (groups with Tasks 2-3)

- [x] 2. Expand Language Enum to 24 Variants

  **What to do**:
  - Expand `Language` enum in `src/models/language.rs` to include all 24 languages from `LANGUAGES` array
  - Languages: Agda, Bash, C, Cpp, CSharp, Css, EmbeddedTemplate, Go, Haskell, Html, Java, JavaScript, Jsdoc, Json, Julia, Ocaml, Php, Python, Regex, Ruby, Rust, Scala, TypeScript, Tsx, Verilog
  - Update `from_path()` function to map extensions to new language variants
  - Update `as_str()` function for all new variants
  - Write unit tests first (TDD)

  **Must NOT do**:
  - Do NOT change the LANGUAGES array in languages.rs (already correct)
  - Do NOT add new file extensions not already in languages.rs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward enum expansion with clear mapping from existing LANGUAGES array
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Task 5 (needs Language enum)
  - **Blocked By**: None (can start immediately)

  **References**:
  - `src/models/language.rs:5-55` - Current Language enum to expand
  - `src/indexer/languages.rs:92-347` - LANGUAGES array with all 24 languages and extensions
  - Each `ext: [...]` line in languages.rs shows the file extensions to map

  **Acceptance Criteria**:
  - [ ] Test file `src/models/language.rs` tests pass (written first)
  - [ ] Language enum has 24 variants (plus Unknown)
  - [ ] `from_path()` correctly maps all extensions
  - [ ] `as_str()` returns correct string for all variants
  - [ ] `cargo test` passes

  **QA Scenarios**:

  ```
  Scenario: All 24 languages detectable from extensions
    Tool: Bash
    Preconditions: Language enum expanded with all variants
    Steps:
      1. Run `cargo test language::tests`
      2. Verify all extension tests pass
    Expected Result: All language detection tests pass
    Failure Indicators: Any test failure
    Evidence: .sisyphus/evidence/task-2-language-tests.txt
  ```

  **Evidence to Capture**:
  - [ ] `cargo test` output showing all tests pass

  **Commit**: NO (groups with Tasks 1, 3)

- [x] 3. Define DiscoveredFile Struct

  **What to do**:
  - Create `src/indexer/discovered.rs` module
  - Define `DiscoveredFile` struct with fields:
    - `path: PathBuf` - Full path to the discovered file
    - `language: &'static LangInfo` - Reference to language info (from languages.rs)
  - Implement `Debug`, `Clone` derives
  - Write unit tests for struct creation
  - Export from `src/indexer/mod.rs`

  **Must NOT do**:
  - Do NOT add fields beyond path and language
  - Do NOT add methods beyond basic struct definition

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple struct definition with two fields
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Task 4 (needs DiscoveredFile struct)
  - **Blocked By**: None (can start immediately)

  **References**:
  - `src/models/symbol.rs:7-17` - Example struct definition pattern
  - `src/indexer/languages.rs:6-18` - LangInfo struct to reference
  - `src/indexer/mod.rs:1-3` - Module export pattern

  **Acceptance Criteria**:
  - [ ] `src/indexer/discovered.rs` file created
  - [ ] `DiscoveredFile` struct with `path: PathBuf` field
  - [ ] `DiscoveredFile` struct with `language: &'static LangInfo` field
  - [ ] Debug, Clone derives implemented
  - [ ] Unit tests for struct creation pass
  - [ ] Exported from `src/indexer/mod.rs`

  **QA Scenarios**:

  ```
  Scenario: DiscoveredFile struct compiles and is usable
    Tool: Bash
    Preconditions: DiscoveredFile struct defined
    Steps:
      1. Run `cargo test discovered`
      2. Verify struct creation tests pass
    Expected Result: All DiscoveredFile tests pass
    Failure Indicators: Compilation errors or test failures
    Evidence: .sisyphus/evidence/task-3-discovered-struct.txt
  ```

  **Evidence to Capture**:
  - [ ] `cargo test` output showing tests pass

  **Commit**: NO (groups with Tasks 1, 2)

- [x] 4. Implement Walker Module with Ignore Rules

  **What to do**:
  - Create `src/indexer/walker.rs` module
  - Implement `discover_files(root: &Path) -> Result<Vec<DiscoveredFile>, WalkerError>` function
  - Use `ignore::WalkBuilder` for directory traversal
  - Configure WalkBuilder:
    - `.git_ignore(true)` - Respect .gitignore
    - `.git_global(true)` - Respect global gitignore
    - `.git_exclude(true)` - Respect .git/info/exclude
    - `.ignore(true)` - Respect .ignore files
    - `.hidden(true)` - EXCLUDE hidden files/directories
    - `.follow_links(false)` - DO NOT follow symlinks
    - `.add_custom_ignore_filename(".indexignore")` - Custom ignore file
  - Filter results to only include files (not directories)
  - Use `from_path()` from languages.rs to detect language
  - Only include files with recognized languages
  - Sort results by path for deterministic output
  - Write unit tests first (TDD)
  - Define `WalkerError` enum with variants: Io, Ignore, LockPoisoned

  **Must NOT do**:
  - Do NOT parse files with tree-sitter (that's Step 3)
  - Do NOT follow symlinks
  - Do NOT include hidden files/directories
  - Do NOT fail entire walk on single file error (log and continue)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Core implementation with complex logic for ignore rules and error handling
  - **Skills**: []
    - No special skills needed, but needs careful attention to ignore crate semantics

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (sequential with Task 5)
  - **Blocks**: Task 5 (needs walker implementation)
  - **Blocked By**: Task 1 (needs ignore crate), Task 3 (needs DiscoveredFile)

  **References**:
  - `src/indexer/languages.rs:66-77` - `from_extension()` and `from_path()` functions
  - https://docs.rs/ignore/latest/ignore/struct.WalkBuilder.html - WalkBuilder API docs
  - https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs - Ripgrep walker implementation
  - `src/error.rs:3-16` - Error definition pattern using thiserror

  **Acceptance Criteria**:
  - [ ] `src/indexer/walker.rs` file created
  - [ ] `WalkerError` enum defined with Io, Ignore, LockPoisoned variants
  - [ ] `discover_files(root: &Path) -> Result<Vec<DiscoveredFile>, WalkerError>` implemented
  - [ ] WalkBuilder configured with all required options
  - [ ] .gitignore rules respected
  - [ ] .indexignore rules respected
  - [ ] Hidden files excluded
  - [ ] Symlinks not followed
  - [ ] Files sorted by path
  - [ ] Unit tests pass
  - [ ] Exported from `src/indexer/mod.rs`

  **QA Scenarios**:

  ```
  Scenario: Walker respects .gitignore rules
    Tool: Bash
    Preconditions: Test fixture with .gitignore containing "*.log"
    Steps:
      1. Create test fixture: /tmp/test-project/ with files: main.rs, debug.log
      2. Create .gitignore with "*.log"
      3. Run walker on /tmp/test-project/
      4. Verify debug.log is NOT in results
      5. Verify main.rs IS in results
    Expected Result: Only main.rs returned, debug.log excluded
    Failure Indicators: debug.log appears in results
    Evidence: .sisyphus/evidence/task-4-gitignore-respect.txt

  Scenario: Walker respects .indexignore rules
    Tool: Bash
    Preconditions: Test fixture with .indexignore containing "internal/"
    Steps:
      1. Create test fixture: /tmp/test-project/ with files: main.rs, internal/secret.rs
      2. Create .indexignore with "internal/"
      3. Run walker on /tmp/test-project/
      4. Verify internal/secret.rs is NOT in results
      5. Verify main.rs IS in results
    Expected Result: Only main.rs returned, internal/ excluded
    Failure Indicators: Files from internal/ appear in results
    Evidence: .sisyphus/evidence/task-4-indexignore-respect.txt

  Scenario: Walker excludes hidden files
    Tool: Bash
    Preconditions: Test fixture with hidden files
    Steps:
      1. Create test fixture: /tmp/test-project/ with files: main.rs, .hidden/secrets.rs
      2. Run walker on /tmp/test-project/
      3. Verify .hidden/secrets.rs is NOT in results
    Expected Result: Hidden directory excluded
    Failure Indicators: .hidden/ files appear in results
    Evidence: .sisyphus/evidence/task-4-hidden-exclude.txt

  Scenario: Walker handles errors gracefully
    Tool: Bash
    Preconditions: Test fixture with some unreadable files
    Steps:
      1. Create test fixture with mix of readable and problematic files
      2. Run walker
      3. Verify walker continues despite errors
      4. Verify readable files are returned
    Expected Result: Walker doesn't crash, returns valid files
    Failure Indicators: Walker crashes or panics
    Evidence: .sisyphus/evidence/task-4-error-handling.txt
  ```

  **Evidence to Capture**:
  - [ ] `cargo test` output showing tests pass
  - [ ] Integration test output with fixture project

  **Commit**: NO (groups with Tasks 1-3, 5-7)

- [x] 5. Integrate Language Detection with Walker

  **What to do**:
  - Update walker to use `from_path()` from `languages.rs` for language detection
  - Filter out files with unrecognized extensions (no matching LangInfo)
  - Set `language` field in `DiscoveredFile` from detected `LangInfo`
  - Ensure consistent language detection between walker and existing `from_path()`
  - Write integration tests covering all 24 languages
  - Verify extension-to-language mapping matches LANGUAGES array

  **Must NOT do**:
  - Do NOT add new language detection logic (use existing from_path)
  - Do NOT include files with unrecognized extensions

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Integration work requiring careful coordination between walker and language modules
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (after Task 4)
  - **Blocks**: Task 6 (needs integrated walker)
  - **Blocked By**: Task 2 (needs expanded Language), Task 4 (needs walker base)

  **References**:
  - `src/indexer/languages.rs:66-77` - `from_extension()` and `from_path()` functions
  - `src/indexer/languages.rs:92-347` - LANGUAGES array with extension mappings

  **Acceptance Criteria**:
  - [ ] Walker uses `from_path()` for language detection
  - [ ] Files with unrecognized extensions are filtered out
  - [ ] `DiscoveredFile.language` correctly set for all 24 languages
  - [ ] Integration tests cover all 24 language extensions
  - [ ] `cargo test` passes

  **QA Scenarios**:

  ```
  Scenario: All 24 languages detected correctly
    Tool: Bash
    Preconditions: Test fixture with files for all 24 languages
    Steps:
      1. Create test fixture with sample files: test.rs, test.py, test.ts, test.go, etc.
      2. Run walker on fixture
      3. Verify all 24 files are returned with correct language
    Expected Result: All 24 files returned with correct LangInfo
    Failure Indicators: Any language missing or incorrectly detected
    Evidence: .sisyphus/evidence/task-5-all-languages.txt

  Scenario: Files with unknown extensions filtered
    Tool: Bash
    Preconditions: Test fixture with .txt, .md, .unknown files
    Steps:
      1. Create test fixture with files: main.rs, readme.txt, notes.md
      2. Run walker on fixture
      3. Verify only main.rs returned
    Expected Result: Only recognized languages returned
    Failure Indicators: .txt or .md files appear in results
    Evidence: .sisyphus/evidence/task-5-unknown-filter.txt
  ```

  **Evidence to Capture**:
  - [ ] `cargo test` output showing tests pass
  - [ ] Integration test showing all 24 languages detected

  **Commit**: NO (groups with Tasks 1-4, 6-7)

- [x] 6. Integration Test with Fixture Project

  **What to do**:
  - Create `tests/fixtures/` directory with test project structure
  - Include sample .gitignore with various patterns
  - Include sample .indexignore with custom patterns
  - Include files that should be:
    - Included (recognized extensions, not ignored)
    - Excluded by .gitignore
    - Excluded by .indexignore
    - Excluded by being hidden
    - Excluded by unrecognized extension
  - Create `tests/integration_test.rs` with comprehensive tests
  - Test all ignore rule scenarios in one integration test
  - Verify deterministic output (sorted by path)

  **Must NOT do**:
  - Do NOT create overly complex fixtures
  - Do NOT include binary files in fixtures

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Integration testing requires careful setup of test fixtures and comprehensive test coverage
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (with Task 7)
  - **Blocks**: Task 7, Final Verification
  - **Blocked By**: Task 5 (needs completed walker)

  **References**:
  - `tests/` directory pattern (create if doesn't exist)
  - https://doc.rust-lang.org/book/ch11-03-test-organization.html - Rust test organization

  **Acceptance Criteria**:
  - [ ] `tests/fixtures/test-project/` created with sample files
  - [ ] `tests/fixtures/test-project/.gitignore` created
  - [ ] `tests/fixtures/test-project/.indexignore` created
  - [ ] `tests/integration_test.rs` created
  - [ ] Test for .gitignore exclusion
  - [ ] Test for .indexignore exclusion
  - [ ] Test for hidden file exclusion
  - [ ] Test for unrecognized extension filtering
  - [ ] Test for deterministic output order
  - [ ] `cargo test` passes all integration tests

  **QA Scenarios**:

  ```
  Scenario: Integration test runs successfully
    Tool: Bash
    Preconditions: Test fixtures and integration tests created
    Steps:
      1. Run `cargo test --test integration_test`
      2. Verify all tests pass
    Expected Result: All integration tests pass
    Failure Indicators: Any test failure
    Evidence: .sisyphus/evidence/task-6-integration-tests.txt

  Scenario: Walker output is deterministic
    Tool: Bash
    Preconditions: Test fixture created
    Steps:
      1. Run walker on test fixture twice
      2. Compare outputs
      3. Verify identical ordering
    Expected Result: Same output order every time
    Failure Indicators: Different ordering between runs
    Evidence: .sisyphus/evidence/task-6-deterministic.txt
  ```

  **Evidence to Capture**:
  - [ ] `cargo test --test integration_test` output
  - [ ] Fixture directory listing

  **Commit**: NO (groups with all tasks)
  - Pre-commit: `cargo test && cargo clippy`

- [x] 7. Update Indexer Module Exports

  **What to do**:
  - Update `src/indexer/mod.rs` to export new modules
  - Export `walker` module
  - Export `discovered` module
  - Re-export `DiscoveredFile` struct at module level
  - Re-export `discover_files` function at module level
  - Re-export `WalkerError` at module level
  - Ensure clean public API: `ctxe::indexer::discover_files()`
  - Verify no clippy warnings about unused exports

  **Must NOT do**:
  - Do NOT export internal implementation details
  - Do NOT create unnecessary re-exports

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple module export updates
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (with Task 6)
  - **Blocks**: Final Verification
  - **Blocked By**: Task 6 (needs all components complete)

  **References**:
  - `src/indexer/mod.rs:1-3` - Current module structure
  - `src/models/mod.rs:7-9` - Example export pattern

  **Acceptance Criteria**:
  - [ ] `src/indexer/mod.rs` updated with `mod walker;`
  - [ ] `src/indexer/mod.rs` updated with `mod discovered;`
  - [ ] `DiscoveredFile` re-exported
  - [ ] `discover_files` re-exported
  - [ ] `WalkerError` re-exported
  - [ ] `cargo clippy` passes with no warnings
  - [ ] Public API accessible via `ctxe::indexer::*`

  **QA Scenarios**:

  ```
  Scenario: Public API exports correctly
    Tool: Bash
    Preconditions: Module exports updated
    Steps:
      1. Run `cargo clippy -- -D warnings`
      2. Verify no unused export warnings
      3. Verify compilation succeeds
    Expected Result: Clean compilation, no warnings
    Failure Indicators: Clippy warnings or compilation errors
    Evidence: .sisyphus/evidence/task-7-exports.txt
  ```

  **Evidence to Capture**:
  - [ ] `cargo clippy` output showing no warnings

  **Commit**: YES (all tasks)
  - Message: `feat(indexer): add file discovery infrastructure`
  - Files: Cargo.toml, language.rs, discovered.rs, walker.rs, mod.rs, tests/
  - Pre-commit: `cargo test && cargo clippy`

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists. For each "Must NOT Have": search codebase for forbidden patterns. Check evidence files. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy --all-targets --all-features -- -D warnings` + `cargo test`. Review all changed files for: `unwrap()` without error handling, commented-out code, unused imports. Check for proper error propagation.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high`
  Create test fixture directory with .gitignore and .indexignore files. Run `cargo test --test integration` or equivalent. Verify walker respects all ignore rules. Test edge cases: empty directory, deeply nested, files matching multiple patterns.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff. Verify 1:1 — everything in spec was built, nothing beyond spec was built. Check "Must NOT do" compliance. Detect task contamination.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **1-3**: `feat(indexer): add file discovery infrastructure` — Cargo.toml, language.rs, discovered.rs, walker.rs
  - Pre-commit: `cargo test && cargo clippy`

---

## Success Criteria

### Verification Commands
```bash
cargo test                          # Expected: All tests pass
cargo clippy -- -D warnings         # Expected: No warnings
cargo build --release               # Expected: Clean build
```

### Final Checklist
- [x] All "Must Have" present
- [x] All "Must NOT Have" absent
- [x] All tests pass
- [x] Walker correctly respects .gitignore
- [x] Walker correctly respects .indexignore
- [x] Hidden files/directories excluded
- [x] Symlinks not followed
- [x] Language enum expanded to 24 variants
