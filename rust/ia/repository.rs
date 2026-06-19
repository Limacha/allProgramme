// db_lib/src/repository.rs
//
// Repository<T> is the generic CRUD handle for any DbRecord type.
//
// Two constructors:
//   Repository::new(db)      → calls ensure_table first  (use at startup)
//   Repository::attached(db) → skips ensure_table        (use at runtime via db.of::<T>())
//
// New method vs previous version:
//   find_many(&[i64]) → fetch a batch of rows by primary key (used for FK resolution)

use crate::database::{
    build_select_cols, column_names, quote_ident, row_to_valueset, validate_identifier, Db,
};
use crate::error::DbError;
use crate::query::QueryBuilder;
use crate::record::DbRecord;
use crate::value::SqlValue;

// ── Repository ────────────────────────────────────────────────────────────────

/// Generic CRUD repository for any type implementing `DbRecord`.
///
/// Cheap to clone — it holds only a `Db` (which is `Arc<Mutex<Connection>>`)
/// and a cached slice of static column names.
///
/// # Two ways to get one
///
/// ```rust
/// // 1. At startup — creates the table if it does not exist
/// let repo = db.repository::<Task>()?;
///
/// // 2. At runtime — lightweight, no SQL, assumes table already exists
/// let repo = db.of::<Task>();
/// ```
#[derive(Clone)]
pub struct Repository<T: DbRecord> {
    pub(crate) db:      Db,
    pub(crate) cols:    Vec<&'static str>,
    pub(crate) _marker: std::marker::PhantomData<T>,
}

impl<T: DbRecord> Repository<T> {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Create the repository AND ensure the table + indexes exist.
    ///
    /// Runs `CREATE TABLE IF NOT EXISTS` — call once at startup, not per frame.
    /// Prefer `db.repository::<T>()` over calling this directly.
    pub fn new(db: Db) -> Result<Self, DbError> {
        db.ensure_table::<T>()?;
        Ok(Self::attached(db))
    }

    /// Create a repository **without** running `ensure_table`.
    ///
    /// Extremely cheap — only clones the Arc and copies static metadata.
    /// Safe to call every frame inside domain repository methods.
    ///
    /// Requires `db.register::<T>()` (or `db.repository::<T>()`) to have been
    /// called at startup, otherwise the first SQL operation will return a
    /// "no such table" error.
    ///
    /// Prefer `db.of::<T>()` over calling this directly.
    pub fn attached(db: Db) -> Self {
        Self {
            cols:    column_names::<T>(),
            db,
            _marker: std::marker::PhantomData,
        }
    }

    // ── WRITE ─────────────────────────────────────────────────────────────────

    /// INSERT a new record. Assigns the DB-generated id via `T::set_id()`.
    /// Returns the new id.
    pub fn insert(&self, mut record: T) -> Result<i64, DbError> {
        let params = record.to_params();
        validate_params(&params)?;
        let (col_sql, placeholders, values) = build_insert_parts(&params);
        let table = quote_ident(T::table_name());
        let sql   = format!("INSERT INTO {table} ({col_sql}) VALUES ({placeholders})");
        let conn  = self.db.lock();
        conn.execute(&sql, rusqlite::params_from_iter(values.iter()))?;
        let id = conn.last_insert_rowid();
        record.set_id(id);
        Ok(id)
    }

    /// UPDATE via a mutation closure: fetch → mutate → write back.
    ///
    /// Returns `Ok(true)` if the row was found, `Ok(false)` otherwise.
    pub fn update(&self, id: i64, mutate: impl FnOnce(&mut T)) -> Result<bool, DbError> {
        let mut record = match self.find(id)? {
            Some(r) => r,
            None    => return Ok(false),
        };
        mutate(&mut record);

        let params = record.to_params();
        validate_params(&params)?;
        let set_clauses: Vec<String> = params.iter()
            .map(|(col, _)| format!("{} = ?", quote_ident(col)))
            .collect();
        let table = quote_ident(T::table_name());
        let sql   = format!(
            "UPDATE {table} SET {} WHERE {} = ?",
            set_clauses.join(", "),
            quote_ident("id"),
        );
        let mut values: Vec<SqlValue> = params.into_iter().map(|(_, v)| v).collect();
        values.push(SqlValue::Integer(id));
        let rows = self.db.lock().execute(&sql, rusqlite::params_from_iter(values.iter()))?;
        Ok(rows > 0)
    }

    /// SOFT DELETE — sets `deleted = 1`. Keeps tombstone for future sync.
    pub fn delete(&self, id: i64) -> Result<bool, DbError> {
        let table = quote_ident(T::table_name());
        let rows  = self.db.lock().execute(
            &format!("UPDATE {table} SET {} = 1 WHERE {} = ?", quote_ident("deleted"), quote_ident("id")),
            [id],
        )?;
        Ok(rows > 0)
    }

    /// HARD DELETE — physically removes the row. Loses the sync tombstone.
    pub fn delete_hard(&self, id: i64) -> Result<bool, DbError> {
        let table = quote_ident(T::table_name());
        let rows  = self.db.lock().execute(
            &format!("DELETE FROM {table} WHERE {} = ?", quote_ident("id")),
            [id],
        )?;
        Ok(rows > 0)
    }

    // ── READ ──────────────────────────────────────────────────────────────────

    /// Fetch one row by primary key. Returns `None` if not found.
    pub fn find(&self, id: i64) -> Result<Option<T>, DbError> {
        let table  = quote_ident(T::table_name());
        let select = build_select_cols::<T>();
        let sql    = format!("SELECT {select} FROM {table} WHERE {} = ? LIMIT 1", quote_ident("id"));
        let cols   = self.cols.clone();
        let conn   = self.db.lock();
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map([id], |row| row_to_valueset(row, &cols))?;
        match rows.next() {
            None      => Ok(None),
            Some(row) => T::from_values(&row?).map(Some).map_err(|e| DbError::Validation(e.to_string())),
        }
    }

    /// Fetch multiple rows by primary key in one query (`WHERE id IN (…)`).
    ///
    /// Used by domain repositories to resolve FK relationships without N+1 queries.
    ///
    /// ```rust
    /// // Batch-fetch all categories referenced by a list of tasks
    /// let cat_ids: Vec<i64> = tasks.iter().filter_map(|t| t.category_id).collect();
    /// let categories: Vec<Category> = db.of::<Category>().find_many(&cat_ids)?;
    /// ```
    pub fn find_many(&self, ids: &[i64]) -> Result<Vec<T>, DbError> {
        if ids.is_empty() { return Ok(vec![]); }

        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let select = build_select_cols::<T>();
        let table  = quote_ident(T::table_name());
        let sql    = format!("SELECT {select} FROM {table} WHERE {} IN ({placeholders})", quote_ident("id"));

        let params: Vec<SqlValue> = ids.iter().map(|&id| SqlValue::Integer(id)).collect();
        let cols   = self.cols.clone();
        let conn   = self.db.lock();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(
            rusqlite::params_from_iter(params.iter()),
            |row| row_to_valueset(row, &cols),
        )?;
        let mut result = Vec::new();
        for row in rows {
            let vs = row?;
            result.push(T::from_values(&vs).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0, rusqlite::types::Type::Null,
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
                )
            })?);
        }
        Ok(result)
    }

    /// Entry point for chainable queries.
    pub fn query(&self) -> QueryBuilder<T> {
        QueryBuilder::new(self.db.clone())
    }

    // ── BULK ──────────────────────────────────────────────────────────────────

    /// INSERT many records in one transaction. Returns the list of new ids.
    pub fn insert_many(&self, records: Vec<T>) -> Result<Vec<i64>, DbError> {
        let conn = self.db.lock();
        conn.execute_batch("BEGIN;")?;
        let mut ids = Vec::with_capacity(records.len());
        for mut record in records {
            let params = record.to_params();
            validate_params(&params)?;
            let (col_sql, placeholders, values) = build_insert_parts(&params);
            let table = quote_ident(T::table_name());
            let sql   = format!("INSERT INTO {table} ({col_sql}) VALUES ({placeholders})");
            conn.execute(&sql, rusqlite::params_from_iter(values.iter()))?;
            let id = conn.last_insert_rowid();
            record.set_id(id);
            ids.push(id);
        }
        conn.execute_batch("COMMIT;")?;
        Ok(ids)
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn validate_params(params: &[(&'static str, SqlValue)]) -> Result<(), DbError> {
    for (col, _) in params { validate_identifier(col)?; }
    Ok(())
}

fn build_insert_parts(params: &[(&'static str, SqlValue)]) -> (String, String, Vec<SqlValue>) {
    let cols         = params.iter().map(|(c, _)| quote_ident(c)).collect::<Vec<_>>().join(", ");
    let placeholders = params.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let values       = params.iter().map(|(_, v)| v.clone()).collect();
    (cols, placeholders, values)
}
