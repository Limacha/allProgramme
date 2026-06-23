// src/todo/category.rs
//
// Category is a simple grouping label for tasks.
// It is the FK *parent* of Task — register it BEFORE Task in AppState::new().
//
// A task may belong to zero or one category (category_id: Option<i64>).

use db_lib::{ColType, Column, DbError, DbRecord, IndexDef, SqlValue, ValueSet, now};

#[derive(Clone, Debug)]
pub struct Category {
    pub id:         i64,
    pub user_id:    i64,
    pub name:       String,
    /// Optional hex color, e.g. "#FF5733"
    pub color:      Option<String>,
    pub deleted:    bool,
    pub updated_at: i64,
}

impl Category {
    pub fn new(user_id: i64, name: &str) -> Self {
        Self {
            id: 0, user_id,
            name:       name.trim().to_string(),
            color:      None,
            deleted:    false,
            updated_at: now(),
        }
    }
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string()); self
    }
}

impl DbRecord for Category {
    fn table_name() -> &'static str { "categories" }

    fn columns() -> Vec<Column> {
        vec![
            Column::new("user_id",    ColType::Integer).not_null(),
            Column::new("name",       ColType::Text).not_null(),
            // NULL means no color set
            Column::new("color",      ColType::Text),
            Column::new("deleted",    ColType::Integer).not_null().default("0"),
            Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
        ]
    }

    fn indexes() -> Vec<IndexDef> {
        vec![ IndexDef::new(&["user_id", "deleted"]) ]
    }

    fn from_values(v: &ValueSet) -> Result<Self, DbError> {
        Ok(Category {
            id:         v.get("id")?.as_i64()?,
            user_id:    v.get("user_id")?.as_i64()?,
            name:       v.get("name")?.as_text()?,
            color:      v.get("color")?.as_opt_text()?,
            deleted:    v.get("deleted")?.as_bool()?,
            updated_at: v.get("updated_at")?.as_i64()?,
        })
    }

    fn to_params(&self) -> Vec<(&'static str, SqlValue)> {
        vec![
            ("user_id",    self.user_id.into()),
            ("name",       self.name.clone().into()),
            ("color",      self.color.clone().into()),
            ("deleted",    self.deleted.into()),
            ("updated_at", now().into()),
        ]
    }

    fn id(&self)               -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
    fn set_id(&mut self, id: i64)              { self.id = id; }
}
