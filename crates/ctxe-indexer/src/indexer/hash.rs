use std::fs::File;
use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HashError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Computes a merkle hash of the contents of a file.
///
/// Reads the file in 8KB chunks for memory efficiency and returns
/// a SHA-256 hash of the complete file content, encoded as a hex string.
pub fn compute_merkle_hash(path: &Path) -> Result<String, HashError> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;

    const BUFFER_SIZE: usize = 8192;
    let mut buffer = [0u8; BUFFER_SIZE];

    while let Ok(count) = file.read(&mut buffer) {
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path();

        let hash = compute_merkle_hash(path).unwrap();
        let expected =
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        assert_eq!(hash, expected);
    }

    #[test]
    fn test_hello_world() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path();

        File::create(path)
            .unwrap()
            .write_all(b"hello world")
            .unwrap();

        let hash = compute_merkle_hash(path).unwrap();
        let expected =
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";

        assert_eq!(hash, expected);
    }

    #[test]
    fn test_large_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path();

        let mut file = File::create(path).unwrap();
        let data = vec![b'x'; 1024 * 1024];
        file.write_all(&data).unwrap();

        let hash = compute_merkle_hash(path).unwrap();
        let expected =
            "8f990ba0b577b51cf009ea049368c16bbda1b21e1b93be07a824758bb253c39b";

        assert_eq!(hash, expected);
    }
}
