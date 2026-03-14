use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Golang,
    Unknown,
}

impl Language {
    pub fn from_extension(path: &Path) -> Self {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        Self::from_path(ext.to_lowercase().as_str())
    }

    pub fn from_path(path: &str) -> Self {
        match path.rsplit(".").next() {
            Some("rs") => Language::Rust,
            Some("py") => Language::Python,
            Some("js") | Some("mjs") | Some("cjs") | Some("jsx") => {
                Language::JavaScript
            }
            Some("ts") | Some("tsx") => Language::JavaScript,
            Some("go") => Language::Golang,
            _ => Language::Golang,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Golang => "golang",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
            Self::Python => "python",
            Self::Unknown => "unknown",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
