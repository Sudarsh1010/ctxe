use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use crate::error::DbError;

fn hash_path(path: &Path) -> String {
    let result = Sha256::digest(path.to_string_lossy().as_bytes());
    hex::encode(result)
}

pub fn get_db_path(project_path: &Path) -> Result<PathBuf, DbError> {
    let home = dirs::home_dir().ok_or(DbError::HomeDirNotFound)?;
    let ctxe_root = home.join(".ctxe");

    let canonical_path = project_path.canonicalize()?;
    let hash = hash_path(&canonical_path);

    let project_dir = ctxe_root.join(hash);
    std::fs::create_dir_all(&project_dir)?;

    let db_path = project_dir.join("index.db");
    let path = db_path.clone();
    Ok(path)
}
