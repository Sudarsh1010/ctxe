pub const V0: &str = r#"
    CREATE TABLE files (
        id INTEGER PRIMARY KEY,
        path TEXT NOT NULL UNIQUE,
        file_hash TEXT NOT NULL,
        language TEXT NOT NULL,
        last_indexed_at INTEGER NOT NULL
    );
    CREATE TABLE symbols (
        id INTEGER PRIMARY KEY,
        file_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        kind TEXT NOT NULL,
        signature TEXT,
        body TEXT,
        start_line INTEGER NOT NULL,
        end_line INTEGER NOT NULL,
        FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
    );
    CREATE VIRTUAL TABLE symbol_vectors USING vec0(
        symbol_id INTEGER PRIMARY KEY,
        embedding FLOAT[384]
    );
    CREATE TABLE relations (
        id INTEGER PRIMARY KEY,
        source_symbol_id INTEGER NOT NULL,
        target_symbol_id INTEGER,
        target_external_name TEXT,
        relation_type TEXT NOT NULL,
        FOREIGN KEY(source_symbol_id) REFERENCES symbols(id) ON DELETE CASCADE
    );
"#;

/// A list of all migrations in order.
pub const MIGRATIONS: &[&str] = &[V0];
