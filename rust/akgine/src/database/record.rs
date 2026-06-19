// DbRecord is the only trait the application needs to implement to use the
// library. Everything else (table creation, SQL building, row mapping) is
// handled automatically by Repository<T>.
//
// ValueSet is the opaque row handle passed to from_values().
// The app accesses column values by name — no positional indices, no rusqlite.

use crate::database::column::{Column, IndexDef};
use crate::database::error::DbError;
use crate::database::value::SqlValue;

// ── ValueSet ──────────────────────────────────────────────────────────────────

/// An opaque row handle passed to DbRecord::from_values().
///
/// Access values by column name. The `id` column is always available via
/// `get("id")`. All other column names must appear in `DbRecord::columns()`.
///
/// # Example
/// ```rust
/// fn from_values(v: &ValueSet) -> Result<Self, DbError> {
///     Ok(Task {
///         id:      v.get("id")?.as_i64()?,
///         title:   v.get("title")?.as_text()?,
///         done:    v.get("done")?.as_bool()?,
///     })
/// }
/// ```
pub struct ValueSet {
    /// values[0] = id, values[1..] = DbRecord::columns() in order.
    pub(crate) values: Vec<SqlValue>,
    /// names[i] corresponds to values[i + 1].
    pub(crate) column_names: Vec<&'static str>,
}

impl ValueSet {
    pub(crate) fn new(values: Vec<SqlValue>, column_names: Vec<&'static str>) -> Self {
        Self {
            values,
            column_names,
        }
    }

    /// Retrieve a column value by name.
    ///
    /// "id" is always valid. Other names must appear in DbRecord::columns().
    /// Returns DbError::ColumnNotFound if the name is not in this row.
    pub fn get(&self, name: &'static str) -> Result<&SqlValue, DbError> {
        if name == "id" {
            return self
                .values
                .get(0)
                .ok_or_else(|| DbError::ColumnNotFound("id".into()));
        }
        // Linear scan — typically < 20 columns, not a bottleneck.
        let pos = self
            .column_names
            .iter()
            .position(|&n| n == name)
            .ok_or_else(|| DbError::ColumnNotFound(name.into()))?;
        self.values
            .get(pos + 1)
            .ok_or_else(|| DbError::ColumnNotFound(name.into()))
    }
}

// ── DbRecord trait ────────────────────────────────────────────────────────────

/// Trait that makes a struct storable in a SQLite table via Repository<T>.
///
/// # What you implement
/// - `table_name` — the SQLite table name (e.g., `"tasks"`).
/// - `columns` — all columns except `id` (which is auto-managed).
/// - `indexes` — optional composite indexes (default: none).
/// - `from_values` — deserialize one row into Self.
/// - `to_params` — serialize Self into column-value pairs for INSERT/UPDATE.
/// - `id` / `set_id` — let the library manage the primary key.
///
/// # What the library handles automatically
/// - `CREATE TABLE IF NOT EXISTS` with all columns and the `id` PK.
/// - `CREATE INDEX IF NOT EXISTS` for every IndexDef.
/// - Building parameterized INSERT, UPDATE, DELETE, SELECT SQL.
/// - Mapping query results back to Vec<T> via from_values.
///
/// # Security
/// Column names and table names are double-quoted in all generated SQL.
/// All values go through `?` parameterized placeholders — no interpolation.
///
/// # Example implementation
/// ```rust
/// impl DbRecord for Task {
///     fn table_name() -> &'static str { "tasks" }
///
///     fn columns() -> Vec<Column> {
///         vec![
///             Column::new("user_id", ColType::Integer).not_null(),
///             Column::new("title",   ColType::Text).not_null(),
///             Column::new("done",    ColType::Integer).not_null().default("0"),
///             Column::new("deleted", ColType::Integer).not_null().default("0"),
///             Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
///         ]
///     }
///
///     fn indexes() -> Vec<IndexDef> {
///         vec![ IndexDef::new(&["user_id", "deleted"]) ]
///     }
///
///     fn from_values(v: &ValueSet) -> Result<Self, DbError> {
///         Ok(Task {
///             id:         v.get("id")?.as_i64()?,
///             user_id:    v.get("user_id")?.as_i64()?,
///             title:      v.get("title")?.as_text()?,
///             done:       v.get("done")?.as_bool()?,
///             deleted:    v.get("deleted")?.as_bool()?,
///             updated_at: v.get("updated_at")?.as_i64()?,
///         })
///     }
///
///     fn to_params(&self) -> Vec<(&'static str, SqlValue)> {
///         vec![
///             ("user_id",    self.user_id.into()),
///             ("title",      self.title.clone().into()),
///             ("done",       self.done.into()),
///             ("deleted",    self.deleted.into()),
///             ("updated_at", db_lib::now().into()),
///         ]
///     }
///
///     fn id(&self)               -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
///     fn set_id(&mut self, id: i64) { self.id = id; }
/// }
/// ```
pub trait DbRecord: Sized + Clone {
    // ── Schema ────────────────────────────────────────────────────────────────

    /// The SQLite table name. Must match `^[A-Za-z][A-Za-z0-9_]*$`.
    fn table_name() -> &'static str;

    /// All columns except `id`.
    ///
    /// The order here defines the order in `from_values()`:
    ///   values[0] = id
    ///   values[1] = columns()[0].name
    ///   values[2] = columns()[1].name
    ///   …
    fn columns() -> Vec<Column>;

    /// Optional composite indexes. Default: none.
    fn indexes() -> Vec<IndexDef> {
        vec![]
    }

    // ── Row mapping ───────────────────────────────────────────────────────────

    /// Deserialize one database row into Self.
    ///
    /// Access columns by name using `ValueSet::get("col")`.
    fn from_values(v: &ValueSet) -> Result<Self, DbError>;

    /// Serialize Self into column-name → value pairs for INSERT and UPDATE.
    ///
    /// - Do NOT include `id` — the library handles the primary key.
    /// - Columns with DEFAULT values that you want auto-applied can be omitted.
    /// - Columns you do include override any DEFAULT.
    ///
    /// For `updated_at`, include it here with `db_lib::now().into()` so it is
    /// stamped correctly on every write.
    fn to_params(&self) -> Vec<(&'static str, SqlValue)>;

    // ── Primary key ───────────────────────────────────────────────────────────

    /// Returns Some(id) for an already-persisted record, None for a new one
    /// that has not been inserted yet (id == 0).
    fn id(&self) -> Option<i64>;

    /// Called by Repository::insert() to assign the DB-generated id to Self.
    fn set_id(&mut self, id: i64);
}
