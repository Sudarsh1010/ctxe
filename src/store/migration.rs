use rusqlite::Connection;

use crate::error::DbError;

use super::schema;

pub fn run_migrations(conn: &mut Connection) -> Result<(), DbError> {
    let current_version: i32 =
        conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let migrations = schema::MIGRATIONS;

    if current_version < migrations.len() as i32 {
        let tx = conn.transaction()?;

        for (i, mig_sql) in migrations.iter().enumerate() {
            let target_version = (i + 1) as i32;

            if current_version < target_version {
                println!(
                    "[store] Migrating database to version {}",
                    target_version
                );

                tx.execute_batch(mig_sql)?;
                tx.pragma_update(None, "user_version", target_version)?;
            }
        }

        tx.commit()?;
    }

    Ok(())
}
