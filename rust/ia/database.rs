// db_lib/src/database.rs
//
// Db is the single connection handle used by the whole application.
//
// Two new methods vs the previous version:
//
//   db.register::<T>()
//     Runs CREATE TABLE IF NOT EXISTS once at startup — stores nothing.
//     Call this for every domain type before using db.of::<T>().
//
//   db.of::<T>()
//     Returns a lightweight Repository<T> WITHOUT running ensure_table.
//     Safe to call many times per frame because no SQL is executed.
//     Assumes register::<T>() was already called at startup.
//
// Design:
//   AppState stores only Db + user_id.
//   Domain repositories store Db and create sub-repos via db.of::<T>().
//   No Repository<T> ever lives in AppState as a field.

use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use rusqlite::Connection;

use crate::column::Column;
use crate::error::DbError;
use crate::record::DbRecord;

// ── Db ────────────────────────────────────────────────────────────────────────

/// A cheaply cloneable handle to one SQLite database connection.
///
/// Cloning is free — it only increments an Arc counter.
/// All Repository<T> and domain repositories hold a clone of this handle.
#[derive(Clone)]
pub struct Db {
    inner: Arc<Mutex<Connection>>,
}

impl Db {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Open or create the SQLite file at `path`.
    pub fn open(path: &Path) -> Result<Self, DbError> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(path)?;
        configure(&conn)?;
        Ok(Self { inner: Arc::new(Mutex::new(conn)) })
    }

    /// In-memory database — useful for unit tests.
    pub fn in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        configure(&conn)?;
        Ok(Self { inner: Arc::new(Mutex::new(conn)) })
    }

    // ── Table registration ────────────────────────────────────────────────────

    /// Ensure the table for `T` exists (CREATE TABLE / INDEX IF NOT EXISTS).
    ///
    /// Call this **once per domain type at startup**, before using `of::<T>()`.
    ///
    /// FK rule: register the parent table before the child table.
    ///   `db.register::<Category>()? ` → then → `db.register::<Task>()?`
    ///
    /// This is idempotent — safe to call more than once.
    pub fn register<T: DbRecord>(&self) -> Result<(), DbError> {
        self.ensure_table::<T>()
    }

    // ── Repository factory ────────────────────────────────────────────────────

    /// Get a `Repository<T>` **without** running ensure_table.
    ///
    /// Cheap — only clones the Arc handle and copies static metadata.
    /// Safe to call every frame.
    ///
    /// Requires `db.register::<T>()` to have been called at startup.
    pub fn of<T: DbRecord>(&self) -> crate::repository::Repository<T> {
        crate::repository::Repository::attached(self.clone())
    }

    /// Create a `Repository<T>` AND ensure the table exists.
    ///
    /// Use when you want a single call that combines registration and access.
    /// Slightly slower than `of::<T>()` on repeated calls.
    pub fn repository<T: DbRecord>(&self) -> Result<crate::repository::Repository<T>, DbError> {
        crate::repository::Repository::new(self.clone())
    }

    // ── Internal accessors ────────────────────────────────────────────────────

    /// Lock the connection for one operation.
    ///
    /// The guard releases the lock when it drops out of scope.
    /// Never hold it across an `.await` or across UI frames.
    pub(crate) fn lock(&self) -> MutexGuard<'_, Connection> {
        self.inner.lock().expect("DB mutex poisoned — this is a bug")
    }

    /// Ensure the table and indexes for T exist (CREATE TABLE/INDEX IF NOT EXISTS).
    pub(crate) fn ensure_table<T: DbRecord>(&self) -> Result<(), DbError> {
        validate_identifier(T::table_name())?;

        let create_table   = build_create_table::<T>()?;
        let create_indexes = build_create_indexes::<T>()?;

        let conn = self.lock();
        conn.execute_batch(&format!(
            "BEGIN;\n{}\n{}\nCOMMIT;",
            create_table,
            create_indexes.join("\n")
        ))?;
        Ok(())
    }
}

// ── Connection configuration ──────────────────────────────────────────────────

fn configure(conn: &Connection) -> Result<(), DbError> {
    // WAL: better concurrency + crash-safe without fsync on every write.
    conn.pragma_update(None, "journal_mode", "WAL")?;
    // FK constraints are OFF by default in SQLite — must be set per connection.
    conn.pragma_update(None, "foreign_keys", true)?;
    conn.pragma_update(None, "synchronous",  "NORMAL")?;
    Ok(())
}

// ── SQL generation ────────────────────────────────────────────────────────────

/// Double-quote a SQL identifier to prevent keyword conflicts and injection.
pub(crate) fn quote_ident(name: &str) -> String {
    format!(r#""{}""#, name.replace('"', r#""""#))
}

/// Reject identifiers that contain characters outside [A-Za-z0-9_].
pub(crate) fn validate_identifier(name: &str) -> Result<(), DbError> {
    let ok = !name.is_empty()
        && name.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false)
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if ok { Ok(()) } else { Err(DbError::InvalidIdentifier(name.to_string())) }
}

fn build_create_table<T: DbRecord>() -> Result<String, DbError> {
    let table = quote_ident(T::table_name());
    let mut parts = vec![
        format!(r#"{} INTEGER PRIMARY KEY AUTOINCREMENT"#, quote_ident("id"))
    ];
    for col in T::columns() {
        validate_identifier(col.name)?;
        parts.push(col.to_sql_fragment());
    }
    Ok(format!(
        "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
        table,
        parts.join(",\n    ")
    ))
}

fn build_create_indexes<T: DbRecord>() -> Result<Vec<String>, DbError> {
    let table = T::table_name();
    let mut sqls = Vec::new();
    for idx in T::indexes() {
        for col in idx.columns { validate_identifier(col)?; }
        let col_slug    = idx.columns.join("_");
        let prefix      = if idx.unique { "uidx" } else { "idx" };
        let index_name  = format!("{prefix}_{table}_{col_slug}");
        let cols_sql    = idx.columns.iter().map(|c| quote_ident(c)).collect::<Vec<_>>().join(", ");
        let unique_kw   = if idx.unique { "UNIQUE " } else { "" };
        sqls.push(format!(
            "CREATE {unique_kw}INDEX IF NOT EXISTS {} ON {} ({cols_sql});",
            quote_ident(&index_name),
            quote_ident(table),
        ));
    }
    Ok(sqls)
}

// ── SELECT helpers (used by repository + query) ───────────────────────────────

/// Build `"id", "col1", "col2", …` for SELECT statements.
pub(crate) fn build_select_cols<T: DbRecord>() -> String {
    let mut cols = vec![quote_ident("id")];
    for col in T::columns() { cols.push(quote_ident(col.name)); }
    cols.join(", ")
}

/// Extract static column names (without id) from T::columns().
pub(crate) fn column_names<T: DbRecord>() -> Vec<&'static str> {
    T::columns().iter().map(|c| c.name).collect()
}

/// Convert one rusqlite row into a ValueSet, reading `num_cols` columns.
pub(crate) fn row_to_valueset(
    row:          &rusqlite::Row<'_>,
    column_names: &[&'static str],
) -> rusqlite::Result<crate::record::ValueSet> {
    let num = 1 + column_names.len();
    let raw: rusqlite::Result<Vec<rusqlite::types::Value>> =
        (0..num).map(|i| row.get::<_, rusqlite::types::Value>(i)).collect();
    raw.map(|vals| crate::record::ValueSet::new(
        vals.into_iter().map(crate::value::from_rusqlite).collect(),
        column_names.to_vec(),
    ))
}

// ── Utility ───────────────────────────────────────────────────────────────────

/// Current Unix timestamp in seconds (no extra crates needed).
///
/// Use in `DbRecord::to_params()` for `updated_at` columns:
/// ```rust
/// ("updated_at", now().into())
/// ```
pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
