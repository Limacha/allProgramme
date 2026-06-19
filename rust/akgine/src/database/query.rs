// db_lib/src/query.rs
//
// QueryBuilder<T> is a chainable, type-safe query API.
// The application never writes SQL strings or uses rusqlite directly.
//
// How it works:
//   Each method appends to an internal list of Filters / OrderClauses.
//   When .fetch(&db) or .count(&db) is called, the builder assembles
//   one parameterized SQL string, executes it, and maps the rows via
//   T::from_values().
//
// All column names go through validate_identifier() before use.
// All values are bound as ? parameters — never interpolated into SQL.
//
// Usage example (from a repository method — the app never writes this SQL):
//
//   db.repository::<Task>()?
//     .query()
//     .where_eq("user_id",  user_id)
//     .where_eq("deleted",  false)
//     .where_eq("done",     false)
//     .order_by("id", Dir::Asc)
//     .limit(50)
//     .fetch()?
// QueryBuilder<T> — chainable, parameterized queries.
// No SQL string ever leaves this file into application code.
//
// New vs previous version:
//   Filter is now an enum (Cmp | In | Null) so where_in() is supported.
//   where_in("category_id", &[1, 2, 3]) → WHERE "category_id" IN (?, ?, ?)

use crate::database::database::{
    DataBase, build_select_cols, column_names, quote_ident, row_to_valueset, validate_identifier,
};
use crate::database::error::DbError;
use crate::database::record::DbRecord;
use crate::database::value::SqlValue;

// ── Direction ─────────────────────────────────────────────────────────────────

/// Sort direction for ORDER BY clauses.
#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Asc,
    Desc,
}

impl Dir {
    fn as_sql(self) -> &'static str {
        match self {
            Dir::Asc => "ASC",
            Dir::Desc => "DESC",
        }
    }
}

// ── Filter ────────────────────────────────────────────────────────────────────

/// One WHERE condition.
///
/// Never constructed directly by application code — use the QueryBuilder
/// methods (where_eq, where_like, …) instead.
/*#[derive(Clone)]
struct Filter {
    /// Already-validated and quoted column name fragment, e.g. `"user_id"`.
    col: String,
    /// SQL operator fragment, e.g. `"="`, `"!="`, `"LIKE"`, `"IS NULL"`.
    op: &'static str,
    /// Bound parameter value, or None for IS NULL / IS NOT NULL.
    val: Option<SqlValue>,
}

impl Filter {
    /// Write the SQL fragment: `"col" OP ?`  or  `"col" IS NULL`.
    fn to_sql(&self) -> String {
        if self.val.is_some() {
            format!("{} {} ?", self.col, self.op)
        } else {
            // IS NULL or IS NOT NULL — no placeholder
            format!("{} {}", self.col, self.op)
        }
    }
}*/

#[derive(Clone)]
enum Filter {
    /// col OP ?   or   col IS NULL / IS NOT NULL
    Cmp {
        col: String,
        op: &'static str,
        val: Option<SqlValue>,
    },
    /// col IN (?, ?, …)
    In { col: String, vals: Vec<SqlValue> },
}

impl Filter {
    fn to_sql(&self) -> String {
        match self {
            Filter::Cmp { col, op, val } => {
                if val.is_some() {
                    format!("{col} {op} ?")
                } else {
                    format!("{col} {op}")
                }
            }
            Filter::In { col, vals } => {
                let ph = vals.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                format!("{col} IN ({ph})")
            }
        }
    }
    fn push_params(&self, out: &mut Vec<SqlValue>) {
        match self {
            Filter::Cmp { val: Some(v), .. } => out.push(v.clone()),
            Filter::Cmp { .. } => {}
            Filter::In { vals, .. } => out.extend(vals.iter().cloned()),
        }
    }
}

// ── OrderClause ───────────────────────────────────────────────────────────────

#[derive(Clone)]
struct OrderClause {
    col: String,
    dir: Dir,
}

// ── QueryBuilder ──────────────────────────────────────────────────────────────

/// Chainable query builder for `Repository<T>`.
///
/// Constructed by `Repository::query()`. All methods take `self` by value
/// so chains are clean:
///
/// ```rust
/// let tasks = repo.query()
///     .where_eq("user_id", 1i64)
///     .where_eq("deleted", false)
///     .where_like("title", "%milk%")
///     .order_by("id", Dir::Asc)
///     .limit(20)
///     .fetch()?;
/// ```
pub struct QueryBuilder<T: DbRecord> {
    db: DataBase,
    filters: Vec<Filter>,
    orders: Vec<OrderClause>,
    limit: Option<i64>,
    offset: Option<i64>,
    /// Cached static column names extracted from T::columns()
    cols: Vec<&'static str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DbRecord> QueryBuilder<T> {
    pub(crate) fn new(db: DataBase) -> Self {
        Self {
            db,
            filters: Vec::new(),
            orders: vec![],
            limit: None,
            offset: None,
            cols: column_names::<T>(),
            _marker: std::marker::PhantomData,
        }
    }
    // ── WHERE filters ─────────────────────────────────────────────────────────

    /// `WHERE "col" = ?`
    ///
    /// Accepts any type that converts to SqlValue: i64, bool, String, &str, f64, …
    pub fn where_eq(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.add_filter(col, "=", Some(val.into()))
    }

    /// `WHERE "col" != ?`
    pub fn where_neq(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.add_filter(col, "!=", Some(val.into()))
    }

    /// `WHERE "col" > ?`
    pub fn where_gt(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.add_filter(col, ">", Some(val.into()))
    }

    /// `WHERE "col" >= ?`
    pub fn where_gte(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.add_filter(col, ">=", Some(val.into()))
    }

    /// `WHERE "col" < ?`
    pub fn where_lt(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.add_filter(col, "<", Some(val.into()))
    }

    /// `WHERE "col" <= ?`
    pub fn where_lte(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.add_filter(col, "<=", Some(val.into()))
    }

    /// `WHERE "col" LIKE ?`  (use % and _ wildcards in the value)
    ///
    /// Example: `.where_like("title", "%milk%")`
    pub fn where_like(self, col: &'static str, pattern: impl Into<SqlValue>) -> Self {
        self.add_filter(col, "LIKE", Some(pattern.into()))
    }

    /// `WHERE "col" IS NULL`
    pub fn where_null(self, col: &'static str) -> Self {
        self.add_filter(col, "IS NULL", None)
    }

    /// `WHERE "col" IS NOT NULL`
    pub fn where_not_null(self, col: &'static str) -> Self {
        self.add_filter(col, "IS NOT NULL", None)
    }

    /// `WHERE "col" IN (?, ?, …)`
    ///
    /// Used to fetch rows linked by FK to a set of parent ids:
    /// ```rust
    /// // All tasks that belong to category 1, 3, or 7
    /// repo.query()
    ///     .where_in("category_id", vec![1, 3, 7])
    ///     .fetch()?;
    /// ```
    ///
    /// An empty slice produces a query that always returns zero rows (no panic).
    pub fn where_in(mut self, col: &'static str, ids: Vec<i64>) -> Self {
        if validate_identifier(col).is_err() {
            return self;
        }
        if ids.is_empty() {
            // WHERE id = -1 is always false → zero rows returned
            return self.add_filter("id", "=", Some(SqlValue::Integer(-1)));
        }
        let vals: Vec<SqlValue> = ids.into_iter().map(SqlValue::Integer).collect();
        self.filters.push(Filter::In {
            col: quote_ident(col),
            vals,
        });
        self
    }

    // ── ORDER BY ──────────────────────────────────────────────────────────────

    /// `ORDER BY "col" ASC|DESC`
    ///
    /// Multiple calls add multiple ORDER BY terms.
    pub fn order_by(mut self, col: &'static str, dir: Dir) -> Self {
        // validate eagerly so the error surfaces at the call site, not at fetch()
        if validate_identifier(col).is_ok() {
            self.orders.push(OrderClause {
                col: quote_ident(col),
                dir,
            });
        }
        self
    }

    // ── LIMIT / OFFSET ────────────────────────────────────────────────────────

    /// `LIMIT n`
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }

    /// `OFFSET n`  (requires LIMIT to be set; ignored by SQLite otherwise)
    pub fn offset(mut self, n: i64) -> Self {
        self.offset = Some(n);
        self
    }

    // ── Terminal operations ───────────────────────────────────────────────────

    /// Execute the query and return all matching rows as `Vec<T>`.
    pub fn fetch(self) -> Result<Vec<T>, DbError> {
        let (sql, params) = self.build_select()?;
        let conn = self.db.lock();
        let cols = self.cols.clone();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            row_to_valueset(row, &cols)
        })?;
        let mut result = Vec::new();
        for row in rows {
            let vs = row?;
            result.push(T::from_values(&vs).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Null,
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )),
                )
            })?);
        }
        Ok(result)
    }

    /// Execute and return only the first matching row, or `None`.
    // pub fn fetch_one(self) -> Result<Option<T>, DbError> {
    //     let mut results = self.limit(1).fetch()?;
    //     Ok(results.pop())
    // }

    /// Execute and return only the first row (adds LIMIT 1).
    pub fn fetch_one(self) -> Result<Option<T>, DbError> {
        Ok(self.limit(1).fetch()?.into_iter().next())
    }

    /// `SELECT COUNT(*) FROM … WHERE …`
    ///
    /// Ignores ORDER BY, LIMIT, and OFFSET — they are irrelevant for counting.
    pub fn count(self) -> Result<i64, DbError> {
        let (where_sql, params) = build_where(&self.filters)?;
        let table = quote_ident(T::table_name());
        let sql = format!("SELECT COUNT(*) FROM {table}{where_sql}");
        let conn = self.db.lock();
        let count = conn.query_row(&sql, rusqlite::params_from_iter(params.iter()), |row| {
            row.get::<_, i64>(0)
        })?;
        Ok(count)
    }

    /// Returns `true` if at least one row matches the filters.
    pub fn exists(self) -> Result<bool, DbError> {
        Ok(self.count()? > 0)
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    // fn add_filter(mut self, col: &'static str, op: &'static str, val: Option<SqlValue>) -> Self {
    //     // Validate eagerly — if invalid, silently skip (error will surface at fetch()).
    //     // We could also store a pending error, but eager validation is simpler.
    //     if let Ok(()) = validate_identifier(col) {
    //         self.filters.push(Filter {
    //             col: quote_ident(col),
    //             op,
    //             val,
    //         });
    //     }
    //     self
    // }

    fn add_filter(mut self, col: &'static str, op: &'static str, val: Option<SqlValue>) -> Self {
        if validate_identifier(col).is_ok() {
            self.filters.push(Filter::Cmp {
                col: quote_ident(col),
                op,
                val,
            });
        }
        self
    }

    fn build_select(&self) -> Result<(String, Vec<SqlValue>), DbError> {
        let table = quote_ident(T::table_name());
        let select = build_select_cols::<T>();
        let (where_sql, params) = build_where(&self.filters)?;

        let order_sql = if self.orders.is_empty() {
            String::new()
        } else {
            let terms: Vec<String> = self
                .orders
                .iter()
                .map(|o| format!("{} {}", o.col, o.dir.as_sql()))
                .collect();
            format!(" ORDER BY {}", terms.join(", "))
        };

        let limit_sql = match (self.limit, self.offset) {
            (Some(l), Some(o)) => format!(" LIMIT {l} OFFSET {o}"),
            (Some(l), None) => format!(" LIMIT {l}"),
            _ => String::new(),
        };

        let sql = format!("SELECT {select} FROM {table}{where_sql}{order_sql}{limit_sql}");
        Ok((sql, params))
    }
}

// ── Shared WHERE builder ──────────────────────────────────────────────────────

/// Build the " WHERE …" fragment and collect bound parameter values.
/// Returns ("", vec![]) when there are no filters.
pub(crate) fn build_where(filters: &[Filter]) -> Result<(String, Vec<SqlValue>), DbError> {
    if filters.is_empty() {
        return Ok((String::new(), vec![]));
    }
    let mut params: Vec<SqlValue> = Vec::new();
    let fragments: Vec<String> = filters
        .iter()
        .map(|f| {
            // if let Some(v) = &f.val {
            //     params.push(v.clone());
            // }
            f.push_params(&mut params);
            f.to_sql()
        })
        .collect();
    Ok((format!(" WHERE {}", fragments.join(" AND ")), params))
}
