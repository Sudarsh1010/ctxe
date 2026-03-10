use sha2::{Digest, Sha256};
use std::path::Path;

pub(crate) fn hash_path(path: &Path) -> String {
    let result = Sha256::digest(path.to_string_lossy().as_bytes());
    hex::encode(result)
}
