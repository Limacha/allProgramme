// src/todo/task.rs
//
// Task is the domain model for one todo entry.
//
// The only thing that must match between columns() / to_params() / from_values()
// is the column NAME — order in columns() defines the index in the ValueSet,
// but from_values() always accesses by name so re-ordering columns() is safe.
use akgine::database::{ColType, Column, DbError, DbRecord, IndexDef, SqlValue, ValueSet};
use akgine::now;

// ── Task struct ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Task {
    /// Primary key (0 means not yet inserted).
    pub id: i64,
    /// Owner — always 1 for a local single-user app; used as a sync/admin key.
    pub user_id: i64,
    pub title: String,
    pub done: bool,
    /// Soft-delete flag: 1 = deleted (kept as tombstone for future sync).
    pub deleted: bool,
    pub updated_at: i64,
}

impl Task {
    /// Build a new, not-yet-inserted task for `user_id`.
    pub fn new(user_id: i64, title: String) -> Self {
        Self {
            id: 0,
            user_id,
            title,
            done: false,
            deleted: false,
            updated_at: now(),
        }
    }
}

// ── DbRecord implementation ───────────────────────────────────────────────────

impl DbRecord for Task {
    // ── Table schema ──────────────────────────────────────────────────────────

    fn table_name() -> &'static str {
        "tasks"
    }

    fn columns() -> Vec<Column> {
        vec![
            Column::new("user_id", ColType::Integer).not_null(),
            Column::new("title", ColType::Text).not_null(),
            Column::new("done", ColType::Integer)
                .not_null()
                .default("0"),
            Column::new("deleted", ColType::Integer)
                .not_null()
                .default("0"),
            Column::new("updated_at", ColType::Integer)
                .not_null()
                .default("(unixepoch())"),
        ]
    }

    fn indexes() -> Vec<IndexDef> {
        vec![
            // Fast lookup: "all non-deleted tasks for user X"
            IndexDef::new(&["user_id", "deleted"]),
        ]
    }

    // ── Row → struct ──────────────────────────────────────────────────────────

    fn from_values(v: &ValueSet) -> Result<Self, DbError> {
        Ok(Task {
            id: v.get("id")?.as_i64()?,
            user_id: v.get("user_id")?.as_i64()?,
            title: v.get("title")?.as_text()?,
            done: v.get("done")?.as_bool()?,
            deleted: v.get("deleted")?.as_bool()?,
            updated_at: v.get("updated_at")?.as_i64()?,
        })
    }

    // ── struct → row ──────────────────────────────────────────────────────────

    fn to_params(&self) -> Vec<(&'static str, SqlValue)> {
        vec![
            ("user_id", self.user_id.into()),
            ("title", self.title.clone().into()),
            ("done", self.done.into()),
            ("deleted", self.deleted.into()),
            // Always stamp updated_at with the current time on every write.
            ("updated_at", now().into()),
        ]
    }

    // ── Primary key ───────────────────────────────────────────────────────────

    fn id(&self) -> Option<i64> {
        if self.id > 0 { Some(self.id) } else { None }
    }
    fn set_id(&mut self, id: i64) {
        self.id = id;
    }
}
