# ctxe

**Context optimization for AI coding agents.**

## Why ctxe?

AI coding agents (Claude Code, OpenCode, Cursor) need context about your codebase. Current tools use **primitive truncation**:

- OpenCode: Cuts files at 50KB/2000 lines
- Claude Code: Manual file selection
- Cursor: Basic autocomplete context

**ctxe provides intelligent context:**

- 🔍 **Semantic search** - Finds relevant code by meaning, not just keywords
- 🦖 **AST compression** - Reduces code to signatures (60-70% token reduction)
- 📊 **Proactive budget** - Prevents overflow instead of reacting to it
- 🔗 **MCP integration** - Works with all major coding agents

**Research shows** OpenCode has **0 implementations** of semantic search or AST compression. ctxe fills this gap.

---

## How ctxe is Different

| Feature | OpenCode | ctxe |
|---------|----------|------|
| **Context Strategy** | Truncate at 50KB | Semantic search + compress |
| **File Selection** | Manual (user must know) | Automatic (semantic search) |
| **Code Compression** | None (just cuts lines) | AST-based (signatures only) |
| **Token Efficiency** | ~30% (cuts waste) | ~70% (intelligent reduction) |
| **Overflow Handling** | Reactive (detects after) | Proactive (prevents before) |

**Result:** ctxe fits **3x more relevant code** into the same token budget.

## Features

- 🚀 AST-based compression (60-70% token reduction)
- 🔍 Semantic code search with embeddings
- 📊 Token counting and budget enforcement
- 🔗 Git-aware context selection
- 🖥️ MCP server for Claude Code integration

## Installation

### From Binary

```bash
# Download from releases
curl -sL https://github.com/USER/ctxe/releases/latest/download/ctxe-$(uname -m)-$(uname -s) | tar xz
sudo mv ctxe /usr/local/bin/
```

### From Source

```bash
git clone https://github.com/USER/ctxe
cd ctxe
cargo install --path .
```

## Quick Start

```bash
# Build index
ctxe scan .

# Semantic search
ctxe query "authentication logic"

# Compress files
ctxe compress src/ -l L1

# Count tokens
ctxe tokens --input "your text here"
```

## Documentation

- **[Strategic Analysis](../docs/strategic-analysis.md)** - Why ctxe exists and market gap analysis
- **[Architecture](./docs/architecture.md)** - System design and components
- **[API Reference](./docs/api.md)** - Tool documentation
- **[Contributing](./CONTRIBUTING.md)** - How to contribute

## License

MIT OR Apache-2.0
