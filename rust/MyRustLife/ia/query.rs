// db_lib/src/query.rs
//
// QueryBuilder<T> — chainable, parameterized queries.
// No SQL string ever leaves this file into application code.
//
// New vs previous version:
//   Filter is now an enum (Cmp | In | Null) so where_in() is supported.
//   where_in("category_id", &[1, 2, 3]) → WHERE "category_id" IN (?, ?, ?)

use crate::database::{
    build_select_cols, column_names, quote_ident, row_to_valueset, validate_identifier, Db,
};
use crate::error::DbError;
use crate::record::DbRecord;
use crate::value::SqlValue;

// ── Dir ───────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub enum Dir { Asc, Desc }
impl Dir {
    fn as_sql(self) -> &'static str { match self { Dir::Asc => "ASC", Dir::Desc => "DESC" } }
}

// ── Filter ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
enum Filter {
    /// col OP ?   or   col IS NULL / IS NOT NULL
    Cmp { col: String, op: &'static str, val: Option<SqlValue> },
    /// col IN (?, ?, …)
    In  { col: String, vals: Vec<SqlValue> },
}

impl Filter {
    fn to_sql(&self) -> String {
        match self {
            Filter::Cmp { col, op, val } => {
                if val.is_some() { format!("{col} {op} ?") }
                else             { format!("{col} {op}") }
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
            Filter::Cmp { .. }               => {}
            Filter::In  { vals, .. }         => out.extend(vals.iter().cloned()),
        }
    }
}

#[derive(Clone)]
struct OrderClause { col: String, dir: Dir }

// ── QueryBuilder ──────────────────────────────────────────────────────────────

pub struct QueryBuilder<T: DbRecord> {
    db:      Db,
    filters: Vec<Filter>,
    orders:  Vec<OrderClause>,
    limit:   Option<i64>,
    offset:  Option<i64>,
    cols:    Vec<&'static str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DbRecord> QueryBuilder<T> {
    pub(crate) fn new(db: Db) -> Self {
        Self {
            db, filters: vec![], orders: vec![],
            limit: None, offset: None,
            cols: column_names::<T>(),
            _marker: std::marker::PhantomData,
        }
    }

    // ── WHERE filters ─────────────────────────────────────────────────────────

    /// `WHERE "col" = ?`
    pub fn where_eq(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.cmp(col, "=", Some(val.into()))
    }
    /// `WHERE "col" != ?`
    pub fn where_neq(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.cmp(col, "!=", Some(val.into()))
    }
    /// `WHERE "col" > ?`
    pub fn where_gt(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.cmp(col, ">", Some(val.into()))
    }
    /// `WHERE "col" >= ?`
    pub fn where_gte(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.cmp(col, ">=", Some(val.into()))
    }
    /// `WHERE "col" < ?`
    pub fn where_lt(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.cmp(col, "<", Some(val.into()))
    }
    /// `WHERE "col" <= ?`
    pub fn where_lte(self, col: &'static str, val: impl Into<SqlValue>) -> Self {
        self.cmp(col, "<=", Some(val.into()))
    }
    /// `WHERE "col" LIKE ?`  — put `%` in the pattern yourself.
    pub fn where_like(self, col: &'static str, pattern: impl Into<SqlValue>) -> Self {
        self.cmp(col, "LIKE", Some(pattern.into()))
    }
    /// `WHERE "col" IS NULL`
    pub fn where_null(self, col: &'static str) -> Self {
        self.cmp(col, "IS NULL", None)
    }
    /// `WHERE "col" IS NOT NULL`
    pub fn where_not_null(self, col: &'static str) -> Self {
        self.cmp(col, "IS NOT NULL", None)
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
        if validate_identifier(col).is_err() { return self; }
        if ids.is_empty() {
            // WHERE id = -1 is always false → zero rows returned
            return self.cmp("id", "=", Some(SqlValue::Integer(-1)));
        }
        let vals: Vec<SqlValue> = ids.into_iter().map(SqlValue::Integer).collect();
        self.filters.push(Filter::In { col: quote_ident(col), vals });
        self
    }

    // ── ORDER BY ──────────────────────────────────────────────────────────────

    pub fn order_by(mut self, col: &'static str, dir: Dir) -> Self {
        if validate_identifier(col).is_ok() {
            self.orders.push(OrderClause { col: quote_ident(col), dir });
        }
        self
    }

    // ── LIMIT / OFFSET ────────────────────────────────────────────────────────

    pub fn limit(mut self, n: i64)  -> Self { self.limit  = Some(n); self }
    pub fn offset(mut self, n: i64) -> Self { self.offset = Some(n); self }

    // ── Terminal operations ───────────────────────────────────────────────────

    /// Execute and return all matching rows.
    pub fn fetch(self) -> Result<Vec<T>, DbError> {
        let (sql, params) = self.build_select()?;
        let cols = self.cols.clone();
        let conn = self.db.lock();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            row_to_valueset(row, &cols)
        })?;
        let mut result = Vec::new();
        for row in rows {
            let vs = row?;
            result.push(T::from_values(&vs).map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                0, rusqlite::types::Type::Null,
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
            ))?);
        }
        Ok(result)
    }

    /// Execute and return only the first row (adds LIMIT 1).
    pub fn fetch_one(self) -> Result<Option<T>, DbError> {
        Ok(self.limit(1).fetch()?.into_iter().next())
    }

    /// `SELECT COUNT(*) …`
    pub fn count(self) -> Result<i64, DbError> {
        let (where_sql, params) = build_where_clause(&self.filters)?;
        let sql = format!("SELECT COUNT(*) FROM {}{where_sql}", quote_ident(T::table_name()));
        let n = self.db.lock().query_row(
            &sql,
            rusqlite::params_from_iter(params.iter()),
            |r| r.get::<_, i64>(0),
        )?;
        Ok(n)
    }

    /// Returns `true` if at least one row matches.
    pub fn exists(self) -> Result<bool, DbError> {
        Ok(self.count()? > 0)
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    fn cmp(mut self, col: &'static str, op: &'static str, val: Option<SqlValue>) -> Self {
        if validate_identifier(col).is_ok() {
            self.filters.push(Filter::Cmp { col: quote_ident(col), op, val });
        }
        self
    }

    fn build_select(&self) -> Result<(String, Vec<SqlValue>), DbError> {
        let table  = quote_ident(T::table_name());
        let select = build_select_cols::<T>();
        let (where_sql, params) = build_where_clause(&self.filters)?;

        let order_sql = if self.orders.is_empty() {
            String::new()
        } else {
            let terms = self.orders.iter()
                .map(|o| format!("{} {}", o.col, o.dir.as_sql()))
                .collect::<Vec<_>>()
                .join(", ");
            format!(" ORDER BY {terms}")
        };

        let limit_sql = match (self.limit, self.offset) {
            (Some(l), Some(o)) => format!(" LIMIT {l} OFFSET {o}"),
            (Some(l), None)    => format!(" LIMIT {l}"),
            _                  => String::new(),
        };

        Ok((format!("SELECT {select} FROM {table}{where_sql}{order_sql}{limit_sql}"), params))
    }
}

// ── Shared WHERE builder (also used by repository count helper) ───────────────

pub(crate) fn build_where_clause(filters: &[Filter]) -> Result<(String, Vec<SqlValue>), DbError> {
    if filters.is_empty() { return Ok((String::new(), vec![])); }
    let mut params = Vec::new();
    let fragments: Vec<String> = filters.iter().map(|f| {
        f.push_params(&mut params);
        f.to_sql()
    }).collect();
    Ok((format!(" WHERE {}", fragments.join(" AND ")), params))
}
