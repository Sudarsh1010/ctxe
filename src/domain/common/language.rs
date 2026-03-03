use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Go,
    TypeScript,
    JavaScript,
    Python,
    Unknown,
}

impl Language {
    pub fn from_extension(path: &Path) -> Self {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        match ext.to_lowercase().as_str() {
            "rs" => Self::Rust,
            "go" => Self::Go,
            "ts" | "tsx" => Self::TypeScript,
            "js" | "jsx" | "mjs" => Self::JavaScript,
            "py" => Self::Python,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Go => "go",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
            Self::Python => "python",
            Self::Unknown => "unknown",
        }
    }
}
