// SqlValue is the bridge between Rust types and SQLite column values.
//
// The app uses SqlValue in two places:
//   - DbRecord::to_params()  → what to write into the DB
//   - ValueSet::get("col")   → what was read back from the DB
//
// The app never imports rusqlite::types — only SqlValue and DbError.

use crate::database::error::DbError;

#[derive(Clone, Debug, PartialEq)]
pub enum SqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
}

impl SqlValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            SqlValue::Null => "Null",
            SqlValue::Integer(_) => "Integer",
            SqlValue::Real(_) => "Real",
            SqlValue::Text(_) => "Text",
        }
    }

    // ── Typed extractors (used in DbRecord::from_values) ─────────────────────

    /// Extract an i64. Boolean columns stored as 0/1 use as_bool() instead.
    pub fn as_i64(&self) -> Result<i64, DbError> {
        match self {
            SqlValue::Integer(i) => Ok(*i),
            SqlValue::Null => Err(DbError::NullValue("?".into())),
            other => Err(DbError::TypeMismatch {
                column: "?".into(),
                expected: "Integer",
                found: other.type_name(),
            }),
        }
    }

    /// Extract an i64 that is a boolean column (0 = false, anything else = true).
    pub fn as_bool(&self) -> Result<bool, DbError> {
        self.as_i64().map(|i| i != 0)
    }

    /// Extract an f64.
    pub fn as_f64(&self) -> Result<f64, DbError> {
        match self {
            SqlValue::Real(f) => Ok(*f),
            SqlValue::Integer(i) => Ok(*i as f64), // widen integer to float
            SqlValue::Null => Err(DbError::NullValue("?".into())),
            other => Err(DbError::TypeMismatch {
                column: "?".into(),
                expected: "Real",
                found: other.type_name(),
            }),
        }
    }

    /// Extract a String (clones the value).
    pub fn as_text(&self) -> Result<String, DbError> {
        match self {
            SqlValue::Text(s) => Ok(s.clone()),
            SqlValue::Null => Err(DbError::NullValue("?".into())),
            other => Err(DbError::TypeMismatch {
                column: "?".into(),
                expected: "Text",
                found: other.type_name(),
            }),
        }
    }

    /// Extract an optional String (Null → None, Text → Some).
    pub fn as_opt_text(&self) -> Result<Option<String>, DbError> {
        match self {
            SqlValue::Text(s) => Ok(Some(s.clone())),
            SqlValue::Null => Ok(None),
            other => Err(DbError::TypeMismatch {
                column: "?".into(),
                expected: "Text or Null",
                found: other.type_name(),
            }),
        }
    }

    /// Extract an optional i64 (Null → None).
    pub fn as_opt_i64(&self) -> Result<Option<i64>, DbError> {
        match self {
            SqlValue::Integer(i) => Ok(Some(*i)),
            SqlValue::Null => Ok(None),
            other => Err(DbError::TypeMismatch {
                column: "?".into(),
                expected: "Integer or Null",
                found: other.type_name(),
            }),
        }
    }
}

// ── From<T> impls (used in DbRecord::to_params) ───────────────────────────────

impl From<i64> for SqlValue {
    fn from(v: i64) -> Self {
        SqlValue::Integer(v)
    }
}
impl From<i32> for SqlValue {
    fn from(v: i32) -> Self {
        SqlValue::Integer(v as i64)
    }
}
impl From<u32> for SqlValue {
    fn from(v: u32) -> Self {
        SqlValue::Integer(v as i64)
    }
}
impl From<bool> for SqlValue {
    fn from(v: bool) -> Self {
        SqlValue::Integer(v as i64)
    }
}
impl From<f64> for SqlValue {
    fn from(v: f64) -> Self {
        SqlValue::Real(v)
    }
}
impl From<f32> for SqlValue {
    fn from(v: f32) -> Self {
        SqlValue::Real(v as f64)
    }
}
impl From<String> for SqlValue {
    fn from(v: String) -> Self {
        SqlValue::Text(v)
    }
}
impl From<&str> for SqlValue {
    fn from(v: &str) -> Self {
        SqlValue::Text(v.to_string())
    }
}
impl From<Option<String>> for SqlValue {
    fn from(v: Option<String>) -> Self {
        match v {
            Some(s) => SqlValue::Text(s),
            None => SqlValue::Null,
        }
    }
}
impl From<Option<i64>> for SqlValue {
    fn from(v: Option<i64>) -> Self {
        match v {
            Some(i) => SqlValue::Integer(i),
            None => SqlValue::Null,
        }
    }
}

// ── rusqlite ToSql impl (used internally by Repository) ──────────────────────
// This is NEVER exported — it lives entirely inside the lib.

impl rusqlite::types::ToSql for SqlValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value, ValueRef};
        match self {
            SqlValue::Null => Ok(ToSqlOutput::Owned(Value::Null)),
            SqlValue::Integer(i) => Ok(ToSqlOutput::Owned(Value::Integer(*i))),
            SqlValue::Real(f) => Ok(ToSqlOutput::Owned(Value::Real(*f))),
            // Borrow the string bytes — lifetime is tied to &self
            SqlValue::Text(s) => Ok(ToSqlOutput::Borrowed(ValueRef::Text(s.as_bytes()))),
        }
    }
}

// ── Internal conversion FROM rusqlite Value ───────────────────────────────────
// Used by Repository when reading rows. Not exported.

pub(crate) fn from_rusqlite(v: rusqlite::types::Value) -> SqlValue {
    use rusqlite::types::Value::*;
    match v {
        Null => SqlValue::Null,
        Integer(i) => SqlValue::Integer(i),
        Real(f) => SqlValue::Real(f),
        Text(s) => SqlValue::Text(s),
        Blob(_) => SqlValue::Null, // blobs not supported in this lib version
    }
}
