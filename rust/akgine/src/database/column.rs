// Column and IndexDef describe the schema that DbRecord::columns() and
// DbRecord::indexes() return. The library uses them to generate
// CREATE TABLE and CREATE INDEX statements automatically.

// ── ColType ───────────────────────────────────────────────────────────────────

/// The SQLite storage type for a column.
///
/// Note: there is no Boolean type — use Integer and call as_bool() when reading.
/// SQLite stores 0 = false, everything else = true.
#[derive(Clone, Debug)]
pub enum ColType {
    /// INTEGER — i64, bool (0/1), Unix timestamps.
    Integer,
    /// REAL — f64, f32.
    Real,
    /// TEXT — String, enum variants stored as strings.
    Text,
}

impl ColType {
    pub(crate) fn as_sql(&self) -> &'static str {
        match self {
            ColType::Integer => "INTEGER",
            ColType::Real => "REAL",
            ColType::Text => "TEXT",
        }
    }
}

// ── Column ────────────────────────────────────────────────────────────────────

/// Definition of one column in a table (excluding the auto-managed `id` column).
///
/// The library always adds `id INTEGER PRIMARY KEY AUTOINCREMENT` as the first
/// column; you do not include it in `DbRecord::columns()`.
///
/// # Example
/// ```rust
/// fn columns() -> Vec<Column> {
///     vec![
///         Column::new("user_id",    ColType::Integer).not_null(),
///         Column::new("title",      ColType::Text).not_null(),
///         Column::new("done",       ColType::Integer).not_null().default("0"),
///         Column::new("deleted",    ColType::Integer).not_null().default("0"),
///         Column::new("created_at", ColType::Integer).not_null().default("(unixepoch())"),
///         Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
///     ]
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Column {
    /// Column name — must match /^[A-Za-z][A-Za-z0-9_]*$/.
    /// Use the same name in get("col_name") inside from_values().
    pub name: &'static str,

    /// The SQLite storage type.
    pub col_type: ColType,

    /// If true, a NOT NULL constraint is added.
    pub not_null: bool,

    /// Optional DEFAULT expression, written verbatim into the CREATE TABLE SQL.
    /// Examples: "0", "'pending'", "(unixepoch())"
    pub default: Option<&'static str>,
}

impl Column {
    /// Start building a column definition.
    pub const fn new(name: &'static str, col_type: ColType) -> Self {
        Self {
            name,
            col_type,
            not_null: false,
            default: None,
        }
    }

    /// Add NOT NULL constraint.
    pub const fn not_null(mut self) -> Self {
        self.not_null = true;
        self
    }

    /// Add a DEFAULT expression.
    ///
    /// The value is written verbatim into SQL, so include quotes if needed:
    /// ```rust
    /// Column::new("status", ColType::Text).default("'PlanToWatch'")
    /// ```
    pub const fn default(mut self, expr: &'static str) -> Self {
        self.default = Some(expr);
        self
    }

    //r#"..."# -> pas de anti slash requis
    /// Generate the column fragment for CREATE TABLE (without trailing comma).
    pub(crate) fn to_sql_fragment(&self) -> String {
        let mut sql = format!(
            r#""{}" {}"#,
            self.name.replace('"', r#""""#),
            self.col_type.as_sql()
        );
        if self.not_null {
            sql.push_str(" NOT NULL");
        }
        if let Some(def) = self.default {
            sql.push_str(&format!(" DEFAULT {def}"));
        }
        sql
    }
}

// ── IndexDef ──────────────────────────────────────────────────────────────────

/// Definition of a CREATE INDEX statement.
///
/// Return one or more of these from `DbRecord::indexes()` to have the library
/// create the indexes automatically alongside the table.
///
/// # Example
/// ```rust
/// fn indexes() -> Vec<IndexDef> {
///     vec![
///         // WHERE user_id = ? AND deleted = 0 ORDER BY updated_at DESC
///         IndexDef::new(&["user_id", "deleted", "updated_at"]),
///     ]
/// }
/// ```
#[derive(Clone, Debug)]
pub struct IndexDef {
    /// Column names included in the index, in order.
    pub columns: &'static [&'static str],
    /// If true, creates a UNIQUE INDEX instead of a regular one.
    pub unique: bool,
}

impl IndexDef {
    pub const fn new(columns: &'static [&'static str]) -> Self {
        Self {
            columns,
            unique: false,
        }
    }

    pub const fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}
