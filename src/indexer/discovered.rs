use std::path::PathBuf;

use super::languages::LangInfo;

#[derive(Debug, Clone)]
pub struct DiscoveredFile {
    pub path: PathBuf,
    pub language: &'static LangInfo,
}
