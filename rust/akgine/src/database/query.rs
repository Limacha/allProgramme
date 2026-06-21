// All values are bound as ? parameters — never interpolated into SQL.

use crate::database::database::{
    DataBase, column_names, generate_select_columns_sql, quoteIdentifier, row_to_valueset,
    validateIdentifier,
};
use crate::database::error::DbError;
use crate::database::record::DbRecord;
use crate::database::value::SqlValue;

// ── Direction ─────────────────────────────────────────────────────────────────

/// Sort direction for ORDER BY clauses.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Asc,
    Desc,
}

impl Direction {
    fn as_sql(self) -> &'static str {
        match self {
            Direction::Asc => "ASC",
            Direction::Desc => "DESC",
        }
    }
}

// ── Filter ────────────────────────────────────────────────────────────────────

/// One WHERE condition.
///
/// Never constructed directly by application code — use the QueryBuilder
/// methods (where_eq, where_like, …) instead.

#[derive(Clone)]
/// don't execute or generate the final query
/// this just create a template and give all value to finish the template
pub enum Filter {
    /// Used when column validation fails so we can silently ignore it.
    Empty,
    /// column OP ?   or   column IS NULL / IS NOT NULL
    Comparison {
        columnName: String,
        /// =, !=, >, >=, <, <=, LIKE, IS NULL, IS NOT NULL
        operator: &'static str,
        value: Option<SqlValue>,
    },
    /// column IN (?, ?, …)
    Inclusion {
        columnName: String,
        values: Vec<SqlValue>,
    },
    /// (filter1 AND filter2 AND ...)
    And(Vec<Filter>),
    /// (filter1 OR filter2 OR ...)
    Or(Vec<Filter>),
}

impl Filter {
    /// generate a sql query with ? where value need to be
    fn to_sql(&self) -> String {
        match self {
            Filter::Empty => String::new(),
            Filter::Comparison {
                columnName,
                operator,
                value,
            } => {
                if value.is_some() {
                    format!("{columnName} {operator} ?")
                } else {
                    format!("{columnName} {operator}")
                }
            }
            Filter::Inclusion { columnName, values } => {
                let placeHolder: String = values.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                format!("{columnName} IN ({placeHolder})")
            }
            Filter::And(filters) => {
                let frags: Vec<String> = filters
                    .iter()
                    .map(|f| f.to_sql())
                    .filter(|s| !s.is_empty())
                    .collect();

                match frags.len() {
                    0 => String::new(),
                    1 => frags[0].clone(),
                    _ => format!("({})", frags.join(" AND ")),
                }
            }
            Filter::Or(filters) => {
                let frags: Vec<String> = filters
                    .iter()
                    .map(|f| f.to_sql())
                    .filter(|s| !s.is_empty())
                    .collect();

                match frags.len() {
                    0 => String::new(),
                    1 => frags[0].clone(),
                    _ => format!("({})", frags.join(" OR ")),
                }
            }
        }
    }

    /// fill the vec give with the params to replace ? in the query
    fn push_params(&self, out: &mut Vec<SqlValue>) {
        match self {
            Filter::Empty => {}
            Filter::Comparison {
                value: Some(value), ..
            } => out.push(value.clone()),
            Filter::Comparison { .. } => {}
            Filter::Inclusion { values, .. } => out.extend(values.iter().cloned()),
            Filter::And(filters) | Filter::Or(filters) => {
                for f in filters {
                    f.push_params(out);
                }
            }
        }
    }

    /* #region constructeur */

    pub fn eq(columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: "=",
            value: Some(value.into()),
        }
    }

    pub fn neq(columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: "!=",
            value: Some(value.into()),
        }
    }

    pub fn gt(columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: ">",
            value: Some(value.into()),
        }
    }

    pub fn gte(columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: ">=",
            value: Some(value.into()),
        }
    }

    pub fn lt(columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: "<",
            value: Some(value.into()),
        }
    }

    pub fn lte(columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: "<=",
            value: Some(value.into()),
        }
    }

    pub fn like(columnName: &'static str, pattern: impl Into<SqlValue>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: "LIKE",
            value: Some(pattern.into()),
        }
    }

    pub fn null(columnName: &'static str) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: "IS NULL",
            value: None,
        }
    }

    pub fn notNull(columnName: &'static str) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        Filter::Comparison {
            columnName: quoteIdentifier(columnName),
            operator: "IS NOT NULL",
            value: None,
        }
    }

    pub fn in_list(columnName: &'static str, values: Vec<impl Into<SqlValue>>) -> Self {
        if validateIdentifier(columnName).is_err() {
            return Filter::Empty;
        }
        if values.is_empty() {
            return Filter::Comparison {
                columnName: quoteIdentifier("id"),
                operator: "=",
                value: Some(SqlValue::Integer(-1)),
            };
        }
        Filter::Inclusion {
            columnName: quoteIdentifier(columnName),
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }

    pub fn and(filters: Vec<Filter>) -> Self {
        Filter::And(filters)
    }
    pub fn or(filters: Vec<Filter>) -> Self {
        Filter::Or(filters)
    }
    /* #endregion */
}

// ── OrderClause ───────────────────────────────────────────────────────────────

#[derive(Clone)]
struct OrderClause {
    column: String,
    direction: Direction,
}

// ── QueryBuilder ──────────────────────────────────────────────────────────────

/// Chainable query builder for `Repository<T>`.
///
/// Constructed by `Repository::query()`.
///
/// ```rust
/// let tasks = repo.query()
///     .where_eq("user_id", 1i64)
///     .where_eq("deleted", false)
///     .where_like("title", "%milk%")
///     .order_by("id", direction::Asc)
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
    columnNames: Vec<&'static str>,
    /// avoid the compilateur crash for T is never used
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
            columnNames: column_names::<T>(),
            _marker: std::marker::PhantomData,
        }
    }
    /* #region WHERE filters */
    // impl Into<SqlValue> Accepts any type that converts to SqlValue: i64, bool, String, &str, f64, …

    /// Push an arbitrary complex nested Filter
    pub fn where_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    /// `WHERE "columnName" = ?`
    pub fn where_eq(self, columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        self.where_filter(Filter::eq(columnName, value))
    }

    /// `WHERE "columnName" != ?`
    pub fn where_neq(self, columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        self.where_filter(Filter::neq(columnName, value))
    }

    /// `WHERE "columnName" > ?`
    pub fn where_gt(self, columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        self.where_filter(Filter::gt(columnName, value))
    }

    /// `WHERE "columnName" >= ?`
    pub fn where_gte(self, columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        self.where_filter(Filter::gte(columnName, value))
    }

    /// `WHERE "columnName" < ?`
    pub fn where_lt(self, columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        self.where_filter(Filter::lt(columnName, value))
    }

    /// `WHERE "columnName" <= ?`
    pub fn where_lte(self, columnName: &'static str, value: impl Into<SqlValue>) -> Self {
        self.where_filter(Filter::lte(columnName, value))
    }

    /// `WHERE "columnName" LIKE ?`  (use % and _ wildcards in the value)
    ///
    /// Example: `.where_like("title", "%milk%")`
    pub fn where_like(self, columnName: &'static str, pattern: impl Into<SqlValue>) -> Self {
        self.where_filter(Filter::like(columnName, pattern))
    }

    /// `WHERE "columnName" IS NULL`
    pub fn where_null(self, columnName: &'static str) -> Self {
        self.where_filter(Filter::null(columnName))
    }

    /// `WHERE "columnName" IS NOT NULL`
    pub fn where_not_null(self, columnName: &'static str) -> Self {
        self.where_filter(Filter::notNull(columnName))
    }

    /// `WHERE "columnName" IN (?, ?, …)`
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
    pub fn where_in(self, columnName: &'static str, values: Vec<impl Into<SqlValue>>) -> Self {
        self.where_filter(Filter::in_list(columnName, values))
    }
    /* #endregion */

    // ── ORDER BY ──────────────────────────────────────────────────────────────

    /// `ORDER BY "columnName" ASC|DESC`
    ///
    /// Multiple calls add multiple ORDER BY terms.
    pub fn order_by(mut self, columnName: &'static str, direction: Direction) -> Self {
        // validate eagerly so the error surfaces at the call site, not at fetch()
        if validateIdentifier(columnName).is_ok() {
            self.orders.push(OrderClause {
                column: quoteIdentifier(columnName),
                direction: direction,
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

    /// `OFFSET n`  (requires LIMIT to be set; ignored by SQLite otherwise) how many items is skip
    pub fn offset(mut self, n: i64) -> Self {
        self.offset = Some(n);
        self
    }

    // ── Terminal operations ───────────────────────────────────────────────────

    /// Execute the query and return all matching rows as `Vec<T>`.
    pub fn fetch(self) -> Result<Vec<T>, DbError> {
        // get the sql command with ? and the params to replace
        let (sql, params) = self.build_select();
        // lock the db
        let conn: std::sync::MutexGuard<'_, rusqlite::Connection> = self.db.lock();
        // prepare the sql command for rusqlite execution (check syntax, prepare a plan, ...)
        let mut stmt: rusqlite::Statement<'_> = conn.prepare(&sql)?;
        // 1. execute the command whit params give by an iter
        // 2. map each row and change the data in valueSet
        let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            row_to_valueset(row, &self.columnNames)
        })?;

        let mut result: Vec<T> = Vec::new();

        for row in rows {
            let valueSet: super::ValueSet = row?;
            // T::getValues(&valueSet) convert the value in struct
            result.push(T::getValues(&valueSet).map_err(|e| {
                // convert the error in rusqlite error
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

    /// Execute and return only the first row (adds LIMIT 1).
    pub fn fetch_one(self) -> Result<Option<T>, DbError> {
        Ok(self.limit(1).fetch()?.into_iter().next())
    }

    /// `SELECT COUNT(*) FROM … WHERE …`
    ///
    /// Ignores ORDER BY, LIMIT, and OFFSET — they are irrelevant for counting.
    pub fn count(self) -> Result<i64, DbError> {
        let (where_sql, params) = build_where(&self.filters);
        let table: String = quoteIdentifier(T::table_name());
        let sql: String = format!("SELECT COUNT(*) FROM {table} {where_sql}");
        let conn: std::sync::MutexGuard<'_, rusqlite::Connection> = self.db.lock();
        let count: i64 =
            conn.query_row(&sql, rusqlite::params_from_iter(params.iter()), |row| {
                row.get::<_, i64>(0)
            })?;
        Ok(count)
    }

    /// Returns `true` if at least one row matches the filters.
    pub fn exists(self) -> Result<bool, DbError> {
        Ok(self.count()? > 0)
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    fn build_select(&self) -> (String, Vec<SqlValue>) {
        let table: String = quoteIdentifier(T::table_name());
        let select: String = generate_select_columns_sql::<T>();
        let (where_sql, params) = build_where(&self.filters);

        let order_sql = if self.orders.is_empty() {
            String::new()
        } else {
            let terms: Vec<String> = self
                .orders
                .iter()
                .map(|o| format!("{} {}", o.column, o.direction.as_sql()))
                .collect();
            format!(" ORDER BY {}", terms.join(", "))
        };

        let limit_sql = match (self.limit, self.offset) {
            (Some(l), Some(o)) => format!(" LIMIT {l} OFFSET {o}"),
            (Some(l), None) => format!(" LIMIT {l}"),
            _ => String::new(),
        };

        let sql = format!("SELECT {select} FROM {table}{where_sql}{order_sql}{limit_sql}");
        (sql, params)
    }
}

// ── Shared WHERE builder ──────────────────────────────────────────────────────

/// Build the " WHERE …" fragment and collect bound parameter values.
///
/// Join with AND.
///
/// Returns ("", vec![]) when there are no filters.
pub(crate) fn build_where(filters: &[Filter]) -> (String, Vec<SqlValue>) {
    if filters.is_empty() {
        return (String::new(), vec![]);
    }
    let mut params: Vec<SqlValue> = Vec::new();
    let fragments: Vec<String> = filters
        .iter()
        .map(|f| {
            f.push_params(&mut params);
            f.to_sql()
        })
        .filter(|s| !s.is_empty()) // Prevent dangling ANDs or empty strings
        .collect();

    (format!(" WHERE {}", fragments.join(" AND ")), params)
}
