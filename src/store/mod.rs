mod migration;
mod path;
mod schema;

use crate::error::DbError;

use migration::run_migrations;
use rusqlite::Connection;
use rusqlite::ffi::sqlite3_auto_extension;
use sqlite_vec::sqlite3_vec_init;
use std::{mem::transmute, path::Path, sync::Arc};
use tokio::{sync::Mutex, task::spawn_blocking};

pub struct Store {
    pub inner: Arc<Mutex<Connection>>,
}

impl Store {
    pub async fn new(project_path: &Path) -> Result<Self, DbError> {
        let path = path::get_db_path(project_path)?;
        let connection: Connection = spawn_blocking(move || {
            unsafe {
                sqlite3_auto_extension(Some(transmute(
                    sqlite3_vec_init as *const (),
                )));
            }

            let mut conn = Connection::open(&path)?;
            run_migrations(&mut conn)?;

            Ok::<Connection, DbError>(conn)
        })
        .await??;

        let store = Self {
            inner: Arc::new(Mutex::new(connection)),
        };

        Ok(store)
    }
}
