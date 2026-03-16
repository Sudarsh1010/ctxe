use super::discovered::DiscoveredFile;
use super::languages::from_path;
use ignore::WalkBuilder;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum WalkerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ignore error: {0}")]
    Ignore(#[from] ignore::Error),
}

pub fn discover_files(root: &Path) -> Result<Vec<DiscoveredFile>, WalkerError> {
    let mut results = Vec::new();

    let walker = WalkBuilder::new(root)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .hidden(true)
        .follow_links(false)
        .add_custom_ignore_filename(".indexignore")
        .build();

    for entry in walker {
        let entry = entry.map_err(WalkerError::from)?;
        if entry.file_type().is_some_and(|ft| ft.is_file())
            && let Some(lang_info) = from_path(entry.path())
        {
            results.push(DiscoveredFile {
                path: entry.path().to_path_buf(),
                language: lang_info,
            });
        }
    }

    results.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(results)
}
