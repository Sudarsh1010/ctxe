mod path;

use crate::error::DbError;

use path::hash_path;
use rusqlite::Connection;
use rusqlite::ffi::sqlite3_auto_extension;
use sqlite_vec::sqlite3_vec_init;
use std::{
    mem::transmute,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{sync::Mutex, task::spawn_blocking};

struct Store {
    inner: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl Store {
    pub async fn new(project_path: &Path) -> Result<Self, DbError> {
        let home = dirs::home_dir().ok_or(DbError::HomeDirNotFound)?;
        let ctxe_root = home.join(".ctxe");

        let canonical_path = project_path.canonicalize()?;
        let hash = hash_path(&canonical_path);

        let project_dir = ctxe_root.join(hash);
        std::fs::create_dir_all(&project_dir)?;

        let db_path = project_dir.join("index.db");
        let path = db_path.clone();

        let connection: Connection = spawn_blocking(move || {
            unsafe {
                sqlite3_auto_extension(Some(transmute(
                    sqlite3_vec_init as *const (),
                )));
            }

            let conn = Connection::open(&path)?;
            Ok::<Connection, DbError>(conn)
        })
        .await??;

        let store = Self {
            inner: Arc::new(Mutex::new(connection)),
            db_path: ctxe_root,
        };
        Ok(store)
    }
}
