/* Repository<T> is the only type the application uses to write data.
It has no knowledge of any specific domain model — it works for any T
that implements DbRecord.

Public methods:
  insert(&mut self, record: T)  → i64
  update(&self, id, |T| …)      → bool
  delete(&self, id)             → bool  (soft: sets deleted = 1)
  delete_hard(&self, id)        → bool  (removes the row)
  find(&self, id)               → Option<T>
  query(&self)                  → QueryBuilder<T>  (chainable)

No rusqlite type ever appears in the public API.
Column names come from T::columns() — they are validated before any SQL runs. */

use crate::database::database::{DataBase, quoteIdentifier, validateIdentifier};
use crate::database::error::DbError;
use crate::database::query::QueryBuilder;
use crate::database::record::DbRecord;
use crate::database::value::SqlValue;

/* ── Repository ──────────────────────────────────────────────────────────────── */

/**
 Generic CRUD repository for any type implementing `DbRecord`.

 Constructed via `db.repository::<T>()`, which also ensures the table
 and indexes exist before returning.

 `Repository<T>` is `Clone` — cloning it is free (it holds only a `Db`
 which is `Arc<Mutex<Connection>>`).
*/
#[derive(Clone)]
pub struct Repository<T: DbRecord> {
    db: DataBase,
    /* cols: Vec<&'static str>, static column names from T::columns() */
    _marker: std::marker::PhantomData<T>,
}

impl<T: DbRecord> Repository<T> {
    /**
     Create the repository and ensure the table exists.

     Called by `Db::repository::<T>()`. Not meant to be called directly.
    */
    pub(crate) fn new(db: DataBase) -> Result<Self, DbError> {
        db.ensureTable::<T>()?;

        /* check column at the creation */
        for col in T::columns() {
            validateIdentifier(col.name)?;
        }

        Ok(Self::attached(db))
    }

    pub(crate) fn attached(db: DataBase) -> Self {
        /* if no check -> no risk of breach (quoteIdentifier) -> crash made */
        Self {
            /* cols: column_names::<T>(), */
            db,
            _marker: std::marker::PhantomData,
        }
    }

    /* ── WRITE ───────────────────────────────────────────────────────────────── */

    /**
     INSERT a new record.

     The record's `id` field is ignored on entry.
     The DB-generated id is written back via `T::set_id()` and also returned.

     # Example
     ```rust
     let id = repo.insert(Task { id: 0, title: "Buy milk".into(), .. })?;
     ```
    */
    pub fn insert(&self, record: T) -> Result<i64, DbError> {
        let mut ids: Vec<i64> = self.insert_many(vec![record])?;

        ids.pop().ok_or_else(|| DbError::NotFound)
    }

    /**
     UPDATE an existing record by applying a mutation closure.

     The record is first fetched by id. If found, `mutate` is called on it,
     then `to_params()` is called on the mutated record to build the UPDATE.

     Returns `true` if the record was found and updated.

     # Example
     ```rust
     repo.update(42, |task| { task.done = true; })?;
     ```
    */
    pub fn update(&self, id: i64, mutate: impl FnOnce(&mut T)) -> Result<bool, DbError> {
        /* Fetch the current record first */
        let mut record: T = match self.find(id)? {
            Some(r) => r,
            None => return Ok(false),
        };
        mutate(&mut record);

        let params: Vec<(&str, SqlValue)> = record.toParams();
        /* validate_params(&params)?; */

        let set_clauses: Vec<String> = params
            .iter()
            .map(|(col, _)| format!("{} = ?", quoteIdentifier(col)))
            .collect();

        let table: String = quoteIdentifier(T::table_name());
        let sql: String = format!(
            "UPDATE {table} SET {} WHERE {} = ?",
            set_clauses.join(", "),
            quoteIdentifier("id"),
        );

        /* Values: all column values, then the id for the WHERE clause */
        let mut values: Vec<SqlValue> = params.into_iter().map(|(_, v)| v).collect();
        values.push(SqlValue::Integer(id));

        let rows: usize = self
            .db
            .lock()
            .execute(&sql, rusqlite::params_from_iter(values.iter()))?;
        Ok(rows > 0)
    }

    /**
     SOFT DELETE — sets `deleted = 1` on the row.

     Requires the table to have a `deleted` column (INTEGER, default 0).
     Use `query().where_eq("deleted", false)` in normal reads to exclude
     soft-deleted rows.

     Preserves the row as a tombstone for future online sync.

     Returns `true` if the row was found.
    */
    pub fn delete(&self, id: i64) -> Result<bool, DbError> {
        let table: String = quoteIdentifier(T::table_name());
        let rows: usize = self.db.lock().execute(
            &format!(
                "UPDATE {table} SET {} = 1 WHERE {} = ?",
                quoteIdentifier("deleted"),
                quoteIdentifier("id"),
            ),
            [id],
        )?;
        Ok(rows > 0)
    }

    /**
     HARD DELETE — permanently removes the row from the table.

     ⚠️ Removes the tombstone. Use only if you are certain you will never
     sync this record to another device.

     Returns `true` if the row existed.
    */
    pub fn delete_hard(&self, id: i64) -> Result<bool, DbError> {
        let table: String = quoteIdentifier(T::table_name());
        let rows: usize = self.db.lock().execute(
            &format!("DELETE FROM {table} WHERE {} = ?", quoteIdentifier("id")),
            [id],
        )?;
        Ok(rows > 0)
    }

    /* ── READ ────────────────────────────────────────────────────────────────── */

    /**
     Fetch one record by primary key.

     Returns `None` if the id does not exist.
     Does NOT filter by `deleted` — use `query().where_eq("deleted", false)`
     when you want to exclude soft-deleted records.
    */
    pub fn find(&self, id: i64) -> Result<Option<T>, DbError> {
        self.query().where_eq("id", id).fetch_one()
    }

    /**
     Fetch multiple rows by primary key in one query (`WHERE id IN (…)`).

     Used by domain repositories to resolve FK relationships without N+1 queries.

     ```rust
     /* Batch-fetch all categories referenced by a list of tasks */
     let cat_ids: Vec<i64> = tasks.iter().filter_map(|t| t.category_id).collect();
     let categories: Vec<Category> = db.of::<Category>().find_many(&cat_ids)?;
     ```
    */
    pub fn find_many(&self, ids: &[i64]) -> Result<Vec<T>, DbError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        self.query()
            .where_in(
                "id",
                ids.iter().map(|id: &i64| SqlValue::Integer(*id)).collect(),
            )
            .fetch()
    }

    /**
     Entry point for chainable queries.

     Returns a `QueryBuilder<T>` — call `.where_eq(…)`, `.order_by(…)`,
     `.limit(…)`, and finally `.fetch()` or `.count()`.

     # Example
     ```rust
     let pending: Vec<Task> = repo
         .query()
         .where_eq("user_id", user_id)
         .where_eq("done",    false)
         .where_eq("deleted", false)
         .order_by("id", Dir::Asc)
         .fetch()?;
     ```
    */
    pub fn query(&self) -> QueryBuilder<T> {
        QueryBuilder::new(self.db.clone())
    }

    /* ── BULK helpers ────────────────────────────────────────────────────────── */

    /**
     INSERT many records in a single transaction.

     Returns the list of assigned ids in the same order as the input.
    */
    pub fn insert_many(&self, records: Vec<T>) -> Result<Vec<i64>, DbError> {
        let conn = self.db.lock();
        conn.execute_batch("BEGIN;")?;

        let mut ids = Vec::with_capacity(records.len());
        for mut record in records {
            let params = record.toParams();
            /* validate_params(&params)?; */
            let (col_sql, placeholders, values) = generate_insert_query(&params);
            let table = quoteIdentifier(T::table_name());
            let sql = format!("INSERT INTO {table} ({col_sql}) VALUES ({placeholders})");
            conn.execute(&sql, rusqlite::params_from_iter(values.iter()))?;
            let id = conn.last_insert_rowid();
            record.set_id(id);
            ids.push(id);
        }

        conn.execute_batch("COMMIT;")?;
        Ok(ids)
    }
}

/* ── Internal SQL-building helpers ───────────────────────────────────────────── */

/**
 Build the column list, placeholders, and value vector for an INSERT.

 Returns:
  - `"\"col1\", \"col2\""` — for INSERT INTO tbl (…)
  - `"?, ?"` — for VALUES (…)
  - `vec![SqlValue, …]` — the bound parameter values
*/
fn generate_insert_query(params: &[(&'static str, SqlValue)]) -> (String, String, Vec<SqlValue>) {
    let cols: String = params
        .iter()
        .map(|(c, _)| quoteIdentifier(c))
        .collect::<Vec<_>>()
        .join(", ");

    let placeholders: String = params.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    let values: Vec<SqlValue> = params.iter().map(|(_, v)| v.clone()).collect();

    (cols, placeholders, values)
}
