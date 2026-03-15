# Tree-sitter Setup - Learning Notes

## Initial Dependencies

### Phase 1: File Discovery Dependencies
- `ignore = "0.4"`: Gitignore parsing and WalkBuilder (from ripgrep, gold standard for gitignore handling)
- `rayon = "1.10"`: Parallel processing for concurrent file traversal

## Pattern Notes

### Language Enum Expansion Pattern

#### Extension Mapping
1. **Source of Truth**: The `LANGUAGES` array in `src/indexer/languages.rs` (lines 92-347) contains ALL language definitions with their extensions
2. **Mapping to Enum**: Every language variant in `Language` enum must map to the extensions defined in `languages.rs`
3. **from_path() Function**: Must use `match` statement with `rsplit(".").next()` to extract the extension from file paths
4. **Multi-Extension Handling**: Use pattern matching with `|` operator to map multiple extensions to same language
5. **Case-Insensitive**: Convert extension to lowercase for case-insensitive matching

#### Language Variants
- Total languages: 24
- Unknown variant: Always present for unrecognized extensions
- Total enum variants: 25 (24 languages + Unknown)

#### String Representation
- `as_str()` must return lowercase string representation for database storage and serialization
- Each language variant has a unique string identifier (e.g., "agda", "bash", "c", "cpp", "csharp")

#### Test Coverage
- **TDD Approach**: Write tests FIRST for each language's extensions
- **Comprehensive Testing**: Test all extensions for each language variant
- **Edge Cases**: Test file path parsing (no extension, multiple dots)
- **Type Safety**: Use `&Path` for `from_extension()` and `&str` for `from_path()` to prevent type mismatches

#### Common Gotchas
1. **from_extension() expects `&Path`**: Do not pass `PathBuf` directly, use `&PathBuf`
2. **from_path() expects `&str`**: Do not pass `PathBuf` directly, pass string slice
3. **Order Matters**: TypeScript and Tsx extensions are mutually exclusive, must test both separately
4. **Comments are Unnecessary**: Avoid comments in code that is self-documenting (match statements with clear labels)
5. **File Modification Timing**: Always re-read file after modification before editing to avoid "file modified" errors
