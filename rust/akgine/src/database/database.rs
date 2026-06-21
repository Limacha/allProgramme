// Db is the single connection handle used by the whole application.
// It is Arc<Mutex<Connection>> so it is cheap to clone and safe to share.
//
// Responsibilities of THIS file:
//   - Opening the SQLite file and setting connection-level PRAGMAs.
//   - ensure_table<T>(): generating and running CREATE TABLE / CREATE INDEX.
//   - Providing the internal lock() accessor to Repository and QueryBuilder.
//
// What does NOT belong here:
//   - CRUD logic  → repository.rs
//   - Query building → query.rs
//   - Application-specific paths → akTool/src/db/mod.rs

use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use rusqlite::Connection;

// use crate::database::column::Column;
use crate::database::error::DbError;
use crate::database::record::DbRecord;
use crate::database::repository;

// ── Db ────────────────────────────────────────────────────────────────────────

/// A cheaply cloneable handle to one SQLite database connection.
///
/// Cloning `Db` is free — it only increments an Arc reference counter.
/// Store one instance in `AppState`. Pass clones to `Repository<T>`.
///
/// Thread safety: the Mutex ensures only one query runs at a time.
/// This is appropriate for a single-threaded egui application.
#[derive(Clone)]
pub struct DataBase {
    inner: Arc<Mutex<Connection>>,
}

impl DataBase {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Open or create the SQLite file at `path`.
    ///
    /// Creates parent directories automatically.
    /// Sets WAL journal mode and enables foreign-key constraints.
    pub fn open(path: &Path) -> Result<Self, DbError> {
        // verif the dir exist or create it
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok(); // if error ignore it and try open the DB
        }
        // ? -> si erreur stop la fonction et renvoie l'erreur
        let conn: Connection = Connection::open(path)?;
        configure(&conn)?;
        Ok(Self {
            inner: Arc::new(Mutex::new(conn)),
        })
    }

    /// Open an in-memory database. Useful for unit tests.
    /// Don't edit the db file
    /// The database is destroyed when this handle (and all its clones) drop.
    pub fn in_memory() -> Result<Self, DbError> {
        let conn: Connection = Connection::open_in_memory()?;
        configure(&conn)?;
        Ok(Self {
            inner: Arc::new(Mutex::new(conn)),
        })
    }

    // ── Convenience factory ───────────────────────────────────────────────────

    /// Ensure the table for `T` exists (CREATE TABLE / INDEX IF NOT EXISTS).
    ///
    /// Call this **once per domain type at startup**, before using `of::<T>()`.
    ///
    /// FK rule: register the parent table before the child table.
    ///   `db.register::<Category>()? ` → then → `db.register::<Task>()?`
    ///
    /// This is idempotent — safe to call more than once.
    pub fn register<T: DbRecord>(&self) -> Result<(), DbError> {
        self.ensureTable::<T>()
    }

    // ── Repository factory ────────────────────────────────────────────────────

    /// Get a `Repository<T>` **without** running ensure_table.
    ///
    /// Cheap — only clones the Arc handle and copies static metadata.
    /// Safe to call every frame.
    ///
    /// Requires `db.register::<T>()` to have been called at startup.
    pub fn getRepository<T: DbRecord>(&self) -> repository::Repository<T> {
        repository::Repository::attached(self.clone())
    }

    /// Create a `Repository<T>`, ensuring the table and indexes exist.
    ///
    /// This is the main entry-point from application code:
    /// ```rust
    /// let tasks: Repository<Task> = db.repository()?;
    /// ```
    pub fn ensureRepository<T: DbRecord>(&self) -> Result<repository::Repository<T>, DbError> {
        repository::Repository::new(self.clone())
    }

    // ── Internal accessors (used by repository.rs and query.rs) ──────────────

    /// Lock the connection for one operation.
    ///
    /// The `MutexGuard` releases the lock when it drops.
    /// Never hold it across an `.await` or across frames.
    pub(crate) fn lock(&self) -> MutexGuard<'_, Connection> {
        self.inner.lock().expect("DB mutex poisoned")
    }

    /// Create the table and indexes for T if they do not already exist.
    ///
    /// Called once by `Repository::new`. Subsequent calls are no-ops
    /// because every statement uses `IF NOT EXISTS`.
    pub(crate) fn ensureTable<T: DbRecord>(&self) -> Result<(), DbError> {
        validateIdentifier(T::table_name())?;

        let createTableQuery: String = generate_create_table_query::<T>()?;
        let createIndexesQuery: Vec<String> = generate_create_indexes_query::<T>()?;

        let conn: MutexGuard<'_, Connection> = self.lock();

        // Run inside one transaction so partial failure leaves nothing behind.
        conn.execute_batch(&format!(
            "BEGIN;\n{}\n{}\nCOMMIT;",
            createTableQuery,
            createIndexesQuery.join("\n")
        ))?;

        Ok(())
    }
}

// ── Connection configuration ──────────────────────────────────────────────────

/// set the db params
/// journal mode: WAL
/// foreign keys: true
/// synchronous : normal
fn configure(conn: &Connection) -> Result<(), DbError> {
    // WAL: better concurrency, crash-safe without fsync on every write.
    // allow read db while editing in by writing change in .sqlite-wal file
    conn.pragma_update(None, "journal_mode", "WAL")?;
    // Foreign keys are OFF by default in SQLite — always enable.
    conn.pragma_update(None, "foreign_keys", true)?;
    // NORMAL is the safe default for WAL mode.
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    Ok(())
}

// ── SQL generation helpers ────────────────────────────────────────────────────

/// Double-quote a SQL identifier (table name, column name).
///
/// This:
///  1. Prevents SQL keyword conflicts ("select", "from", etc.).
///  2. Handles identifiers that contain special characters.
///  3. Is our defense-in-depth layer (identifiers also pass validate_identifier).
pub(crate) fn quoteIdentifier(name: &str) -> String {
    // SQLite escapes " inside identifiers by doubling it: ""
    format!(r#""{}""#, name.replace('"', r#""""#))
}

/// Validate that an identifier contains only safe characters.
///
/// Allowed: [A-Za-z0-9_], must start with a letter or underscore.
/// Called on table names and column names before any SQL generation.
pub(crate) fn validateIdentifier(name: &str) -> Result<(), DbError> {
    let ok: bool = !name.is_empty()
        && 
        // check the first char
        name
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');

    if ok {
        Ok(())
    } else {
        Err(DbError::InvalidIdentifier(name.to_string()))
    }
}

fn generate_create_table_query<T: DbRecord>() -> Result<String, DbError> {
    let table: String = quoteIdentifier(T::table_name());
    let mut parts: Vec<String> = vec![format!(
        r#"{} INTEGER PRIMARY KEY AUTOINCREMENT"#,
        quoteIdentifier("id")
    )];

    for col in T::columns() {
        validateIdentifier(col.name)?;
        parts.push(col.to_sql_fragment());
    }

    Ok(format!(
        "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
        table,
        parts.join(",\n    ")
    ))
}

fn generate_create_indexes_query<T: DbRecord>() -> Result<Vec<String>, DbError> {
    // don't validate the name because this is already made
    let table: &str = T::table_name();
    let mut sqls: Vec<String> = Vec::new();

    for idx in T::indexes() {
        for col in idx.columns {
            validateIdentifier(col)?;
        }
        let col_slug: String = idx.columns.join("_");
        let index_name: String = if idx.unique {
            format!("uidx_{}_{}", table, col_slug)
        } else {
            format!("idx_{}_{}", table, col_slug)
        };
        let cols_sql: String = idx
            .columns
            .iter()
            .map(|c| quoteIdentifier(c))
            .collect::<Vec<_>>()
            .join(", ");
        let unique: &str = if idx.unique { "UNIQUE " } else { "" };
        sqls.push(format!(
            "CREATE {unique}INDEX IF NOT EXISTS {} ON {} ({cols_sql});",
            quoteIdentifier(&index_name),
            quoteIdentifier(table),
        ));
    }
    Ok(sqls)
}

// ── Internal row-reading helper (shared by repository and query) ──────────────

/// Read all columns for T from one SQLite row, returning a ValueSet.
///
/// The SELECT must use the exact column order: id, then T::columns() in order.
/// `build_select_cols::<T>()` produces the matching SELECT fragment.
pub(crate) fn row_to_valueset(
    row: &rusqlite::Row<'_>,
    column_names: &[&'static str],
) -> rusqlite::Result<crate::database::record::ValueSet> {
    // id + columns set
    let nbColumns: usize = 1 + column_names.len();
    // get each value of the row
    let raw: rusqlite::Result<Vec<rusqlite::types::Value>> = (0..nbColumns)
        .map(|i: usize| row.get::<_, rusqlite::types::Value>(i))
        .collect();

    raw.map(|vals: Vec<rusqlite::types::Value>| {
        crate::database::record::ValueSet::new(
            vals.into_iter()
                .map(crate::database::value::from_rusqlite)
                .collect(),
            column_names.to_vec(),
        )
    })
}

/// Build "id", "col1", "col2", … for SELECT statements.
pub(crate) fn generate_select_columns_sql<T: DbRecord>() -> String {
    let mut columns: Vec<String> = vec![quoteIdentifier("id")];
    for col in T::columns() {
        columns.push(quoteIdentifier(col.name));
    }
    columns.join(", ")
}

/// Extract static column names from T::columns().
pub(crate) fn column_names<T: DbRecord>() -> Vec<&'static str> {
    T::columns().iter().map(|c: &super::Column| c.name).collect()
}
