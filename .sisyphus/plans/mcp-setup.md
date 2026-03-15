# MCP Server Setup

## TL;DR

> **Quick Summary**: Setup minimal MCP server using rmcp crate. No tools/resources/prompts exposed - just infrastructure.
> 
> **Deliverables**:
> - `src/bin/ctxe-mcp/main.rs` - MCP server with stdio transport
> - Server name: `ctxe`
> 
> **Estimated Effort**: Quick
> **Parallel Execution**: NO - single file

---

## Context

### Current State
- `rmcp = { version = "1.2.0", features = ["server"] }` ✓ (in Cargo.toml)
- `src/bin/ctxe-mcp/main.rs` has `todo!()`

### Requirements
- Idiomatic Rust MCP server setup
- stdio transport (standard for MCP servers)
- No tools/resources/prompts exposed (just infrastructure)
- Server name: `ctxe`

---

## TODOs

- [x] 1. Implement MCP Server

  **What to do**:
  - Create `CtxeServer` struct implementing `ServerHandler`
  - Use `ServiceExt::serve(stdio())` pattern
  - Return minimal `ServerInfo` with no capabilities

  **References**:
  - rmcp docs: https://docs.rs/rmcp/latest/rmcp/
  - Pattern: `ServerHandler` trait + `serve(stdio())`

  **Acceptance Criteria**:
  - [x] `cargo build --bin ctxe-mcp` succeeds
  - [x] Binary runs without error (waits for stdio)

  **Commit**: YES
  - Message: `feat(mcp): setup ctxe MCP server skeleton`

---

## Final Verification

- [x] F1. `cargo build --bin ctxe-mcp` succeeds
- [x] F2. Binary compiles and runs

---

## Success Criteria

```bash
cargo build --bin ctxe-mcp  # Builds successfully
cargo run --bin ctxe-mcp    # Runs (waits for stdin)
```
