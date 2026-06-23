// src/todo/task.rs
//
// Task now has an optional FK to categories:
//   category_id: Option<i64>  →  REFERENCES "categories"("id") ON DELETE SET NULL
//
// When a Category is deleted (hard), SQLite automatically sets
// category_id to NULL on all its tasks (because ON DELETE SET NULL).
// When a Category is soft-deleted, the app filters it out in queries.

use db_lib::{ColType, Column, DbError, DbRecord, IndexDef, OnDelete, SqlValue, ValueSet, now};

#[derive(Clone, Debug)]
pub struct Task {
    pub id:          i64,
    pub user_id:     i64,
    /// FK to categories.id  (None = uncategorized)
    pub category_id: Option<i64>,
    pub title:       String,
    pub done:        bool,
    pub deleted:     bool,
    pub updated_at:  i64,
}

impl Task {
    pub fn new(user_id: i64, title: &str) -> Self {
        Self {
            id: 0, user_id,
            category_id: None,
            title:       title.trim().to_string(),
            done:        false,
            deleted:     false,
            updated_at:  now(),
        }
    }
    pub fn in_category(mut self, category_id: i64) -> Self {
        self.category_id = Some(category_id); self
    }
}

impl DbRecord for Task {
    fn table_name() -> &'static str { "tasks" }

    fn columns() -> Vec<Column> {
        vec![
            Column::new("user_id",     ColType::Integer).not_null(),
            // Nullable FK — when the parent Category is hard-deleted,
            // SQLite sets this to NULL automatically (ON DELETE SET NULL).
            Column::new("category_id", ColType::Integer)
                .references("categories", "id")
                .on_delete(OnDelete::SetNull),
            Column::new("title",       ColType::Text).not_null(),
            Column::new("done",        ColType::Integer).not_null().default("0"),
            Column::new("deleted",     ColType::Integer).not_null().default("0"),
            Column::new("updated_at",  ColType::Integer).not_null().default("(unixepoch())"),
        ]
    }

    fn indexes() -> Vec<IndexDef> {
        vec![
            // Fast: WHERE user_id = ? AND deleted = 0
            IndexDef::new(&["user_id", "deleted"]),
            // Fast: WHERE user_id = ? AND category_id = ? AND deleted = 0
            IndexDef::new(&["user_id", "category_id", "deleted"]),
        ]
    }

    fn from_values(v: &ValueSet) -> Result<Self, DbError> {
        Ok(Task {
            id:          v.get("id")?.as_i64()?,
            user_id:     v.get("user_id")?.as_i64()?,
            category_id: v.get("category_id")?.as_opt_i64()?,
            title:       v.get("title")?.as_text()?,
            done:        v.get("done")?.as_bool()?,
            deleted:     v.get("deleted")?.as_bool()?,
            updated_at:  v.get("updated_at")?.as_i64()?,
        })
    }

    fn to_params(&self) -> Vec<(&'static str, SqlValue)> {
        vec![
            ("user_id",     self.user_id.into()),
            ("category_id", self.category_id.into()),   // None → NULL
            ("title",       self.title.clone().into()),
            ("done",        self.done.into()),
            ("deleted",     self.deleted.into()),
            ("updated_at",  now().into()),
        ]
    }

    fn id(&self)               -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
    fn set_id(&mut self, id: i64)              { self.id = id; }
}
