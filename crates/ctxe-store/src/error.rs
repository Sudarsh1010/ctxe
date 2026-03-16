use tokio::task::JoinError;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Join: {0}")]
    Join(#[from] JoinError),

    #[error("Home directory not found")]
    HomeDirNotFound,
}
