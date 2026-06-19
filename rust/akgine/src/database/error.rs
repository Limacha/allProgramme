use std::fmt;

#[derive(Debug)]
pub enum DbError {
    /// A wrapper for SQLite engine error.
    Sql(rusqlite::Error),

    /// A column value had an unexpected SQLite type.
    TypeMismatch {
        column: String,
        expected: &'static str,
        found: &'static str,
    },

    /// `get("col_name")` was called but that name is not in the row.
    ColumnNotFound(String),

    /// A table name or column name contains characters that are not
    /// [a-z A-Z 0-9 _], which the lib requires to build safe SQL.
    InvalidIdentifier(String),

    /// A NOT NULL column contained NULL.
    NullValue(String),

    /// The requested record does not exist.
    NotFound,

    /// An application-level validation failure (empty title, etc.).
    Validation(String),
}

// fonction to know how to display
impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Sql(e) => write!(f, "SQLite: {e}"),
            DbError::TypeMismatch {
                column,
                expected,
                found,
            } => write!(f, "column '{column}': expected {expected}, found {found}"),
            DbError::ColumnNotFound(n) => write!(f, "column not found: '{n}'"),
            DbError::InvalidIdentifier(n) => write!(f, "invalid SQL identifier: '{n}'"),
            DbError::NullValue(col) => write!(f, "column '{col}' is NULL"),
            DbError::NotFound => write!(f, "record not found"),
            DbError::Validation(msg) => write!(f, "validation: {msg}"),
        }
    }
}

// Required so DbError can be boxed in std::error::Error chains.
impl std::error::Error for DbError {
    // use to know if there is a sub-error
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let DbError::Sql(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

// Automatic ? conversion from rusqlite errors inside the lib.
impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError::Sql(e)
    }
}
