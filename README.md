# ctxe

Dynamic context engineering for AI coding agents.

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

- [Architecture](./docs/architecture.md)
- [API Reference](./docs/api.md)
- [Contributing](./CONTRIBUTING.md)

## License

MIT OR Apache-2.0
