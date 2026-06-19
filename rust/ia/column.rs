// db_lib/src/column.rs
//
// Column + IndexDef describe the schema returned by DbRecord::columns()
// and DbRecord::indexes().  The library uses them to generate
// CREATE TABLE and CREATE INDEX automatically.
//
// NEW in this version:
//   - OnDelete enum for FK ON DELETE actions
//   - Column::references(table, col)  →  REFERENCES "tbl"("col")
//   - Column::on_delete(action)       →  ON DELETE CASCADE / SET NULL / …

// ── OnDelete ─────────────────────────────────────────────────────────────────

/// Action taken when the referenced parent row is deleted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnDelete {
    /// Automatically delete child rows.  Most common for owned data.
    Cascade,
    /// Set the FK column to NULL (column must be nullable).
    SetNull,
    /// Reject the parent deletion if any child rows exist.
    Restrict,
    /// Same as Restrict but checked at transaction end.
    NoAction,
}

impl OnDelete {
    pub(crate) fn as_sql(self) -> &'static str {
        match self {
            OnDelete::Cascade  => "CASCADE",
            OnDelete::SetNull  => "SET NULL",
            OnDelete::Restrict => "RESTRICT",
            OnDelete::NoAction => "NO ACTION",
        }
    }
}

// ── ColType ───────────────────────────────────────────────────────────────────

/// SQLite storage type for a column.
///
/// There is no Boolean type — use `Integer` and call `.as_bool()` when reading.
#[derive(Clone, Copy, Debug)]
pub enum ColType {
    Integer,
    Real,
    Text,
}

impl ColType {
    pub(crate) fn as_sql(self) -> &'static str {
        match self {
            ColType::Integer => "INTEGER",
            ColType::Real    => "REAL",
            ColType::Text    => "TEXT",
        }
    }
}

// ── Column ────────────────────────────────────────────────────────────────────

/// Definition of one non-`id` column.
///
/// The library always prepends `id INTEGER PRIMARY KEY AUTOINCREMENT`;
/// never include it in `DbRecord::columns()`.
///
/// # Example
/// ```rust
/// fn columns() -> Vec<Column> {
///     vec![
///         // plain field
///         Column::new("title", ColType::Text).not_null(),
///
///         // nullable FK to another table
///         Column::new("category_id", ColType::Integer)
///             .references("categories", "id")
///             .on_delete(OnDelete::SetNull),
///
///         // non-nullable FK
///         Column::new("project_id", ColType::Integer)
///             .not_null()
///             .references("projects", "id")
///             .on_delete(OnDelete::Cascade),
///     ]
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Column {
    pub name:       &'static str,
    pub col_type:   ColType,
    pub not_null:   bool,
    pub default:    Option<&'static str>,
    /// If set, adds `REFERENCES "table"("col")` to the DDL.
    pub references: Option<(&'static str, &'static str)>,
    /// Action on parent deletion (only meaningful when `references` is set).
    pub on_delete:  Option<OnDelete>,
}

impl Column {
    /// Start building a column definition.
    pub const fn new(name: &'static str, col_type: ColType) -> Self {
        Self {
            name,
            col_type,
            not_null:   false,
            default:    None,
            references: None,
            on_delete:  None,
        }
    }

    /// Add `NOT NULL` constraint.
    pub const fn not_null(mut self) -> Self {
        self.not_null = true;
        self
    }

    /// Add a `DEFAULT` expression (written verbatim into SQL).
    ///
    /// Examples: `"0"`, `"'PlanToWatch'"`, `"(unixepoch())"`.
    pub const fn default(mut self, expr: &'static str) -> Self {
        self.default = Some(expr);
        self
    }

    /// Add a `REFERENCES "other_table"("other_col")` FK constraint.
    ///
    /// The referenced table must be registered **before** this table
    /// (call `db.register::<Parent>()` before `db.register::<Child>()`).
    ///
    /// ⚠️ SQLite only enforces FK constraints when
    /// `PRAGMA foreign_keys = ON` is set — the library always sets this.
    pub const fn references(mut self, table: &'static str, col: &'static str) -> Self {
        self.references = Some((table, col));
        self
    }

    /// Set the `ON DELETE` action for this FK column.
    ///
    /// Only meaningful when `.references()` is also set.
    pub const fn on_delete(mut self, action: OnDelete) -> Self {
        self.on_delete = Some(action);
        self
    }

    /// Generate the SQL fragment for this column (used inside CREATE TABLE).
    pub(crate) fn to_sql_fragment(&self) -> String {
        let type_sql   = self.col_type.as_sql();
        let not_null   = if self.not_null { " NOT NULL" } else { "" };
        let default    = self.default
            .map(|d| format!(" DEFAULT {d}"))
            .unwrap_or_default();
        let references = match self.references {
            None => String::new(),
            Some((tbl, col)) => {
                let on_del = self.on_delete
                    .map(|a| format!(" ON DELETE {}", a.as_sql()))
                    .unwrap_or_default();
                // Double-quote table and column names for safety
                format!(
                    r#" REFERENCES "{}"("{}"){}"#,
                    tbl.replace('"', r#""""#),
                    col.replace('"', r#""""#),
                    on_del
                )
            }
        };

        format!(
            r#""{}" {}{}{}{}"#,
            self.name.replace('"', r#""""#),
            type_sql,
            not_null,
            default,
            references,
        )
    }
}

// ── IndexDef ──────────────────────────────────────────────────────────────────

/// A `CREATE INDEX` definition returned by `DbRecord::indexes()`.
#[derive(Clone, Debug)]
pub struct IndexDef {
    pub columns: &'static [&'static str],
    pub unique:  bool,
}

impl IndexDef {
    pub const fn new(columns: &'static [&'static str]) -> Self {
        Self { columns, unique: false }
    }

    pub const fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}
