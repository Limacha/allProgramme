// src/todo/repository.rs
//
// TodoRepository stores a Db handle, NOT pre-built Repository<T> fields.
// Sub-repositories are created on demand via db.of::<T>() — cheap (Arc clone).
//
// FK pattern:
//   all_with_category() fetches tasks then batch-fetches their categories
//   in ONE extra query (no N+1 problem).
//
// No rusqlite import. No sql strings. Only db_lib types.

use std::collections::HashMap;

use db_lib::{Db, DbError, Dir};

use super::category::Category;
use super::task::Task;

// ── TodoRepository ────────────────────────────────────────────────────────────

pub struct TodoRepository {
    db:      Db,
    user_id: i64,
}

impl TodoRepository {
    /// Construct from a Db handle and the active user id.
    /// Called by AppState::todo() — cheap (Db is just an Arc clone).
    pub fn new(db: Db, user_id: i64) -> Self {
        Self { db, user_id }
    }

    // ── Private sub-repo accessors ────────────────────────────────────────────

    fn tasks(&self)      -> db_lib::Repository<Task>     { self.db.of::<Task>() }
    fn categories(&self) -> db_lib::Repository<Category> { self.db.of::<Category>() }

    // ══════════════════════════════════════════════════════════════════════════
    // CATEGORIES
    // ══════════════════════════════════════════════════════════════════════════

    pub fn add_category(&self, name: &str) -> Result<i64, DbError> {
        let name = name.trim();
        if name.is_empty() { return Err(DbError::Validation("name cannot be empty".into())); }
        self.categories().insert(Category::new(self.user_id, name))
    }

    pub fn add_category_with_color(&self, name: &str, color: &str) -> Result<i64, DbError> {
        let name = name.trim();
        if name.is_empty() { return Err(DbError::Validation("name cannot be empty".into())); }
        self.categories().insert(Category::new(self.user_id, name).with_color(color))
    }

    pub fn rename_category(&self, id: i64, new_name: &str) -> Result<bool, DbError> {
        let n = new_name.trim().to_string();
        self.categories().update(id, move |c| { c.name = n; })
    }

    /// Soft-delete a category.
    /// Tasks that belong to it keep their category_id (pointing to a deleted
    /// category). Use pending_by_category() to filter them as "Uncategorized".
    pub fn remove_category(&self, id: i64) -> Result<bool, DbError> {
        self.categories().delete(id)
    }

    pub fn all_categories(&self) -> Vec<Category> {
        self.categories().query()
            .where_eq("user_id", self.user_id)
            .where_eq("deleted", false)
            .order_by("name", Dir::Asc)
            .fetch()
            .unwrap_or_default()
    }

    // ══════════════════════════════════════════════════════════════════════════
    // TASKS
    // ══════════════════════════════════════════════════════════════════════════

    // ── WRITE ─────────────────────────────────────────────────────────────────

    /// Add a task with no category.
    pub fn add(&self, title: &str) -> Result<i64, DbError> {
        let title = title.trim();
        if title.is_empty() { return Err(DbError::Validation("title cannot be empty".into())); }
        self.tasks().insert(Task::new(self.user_id, title))
    }

    /// Add a task and assign it to an existing category.
    ///
    /// Validates that the category belongs to this user before inserting.
    pub fn add_to_category(&self, title: &str, category_id: i64) -> Result<i64, DbError> {
        let title = title.trim();
        if title.is_empty() { return Err(DbError::Validation("title cannot be empty".into())); }

        // Verify the category exists and belongs to this user
        let cat = self.categories().find(category_id)?
            .ok_or(DbError::NotFound)?;
        if cat.user_id != self.user_id || cat.deleted {
            return Err(DbError::Validation("category not found for this user".into()));
        }

        self.tasks().insert(Task::new(self.user_id, title).in_category(category_id))
    }

    /// Move a task to a different category (or pass None to uncategorize).
    pub fn set_category(&self, task_id: i64, category_id: Option<i64>) -> Result<bool, DbError> {
        self.tasks().update(task_id, move |t| { t.category_id = category_id; })
    }

    pub fn toggle(&self, id: i64)  -> Result<bool, DbError> {
        self.tasks().update(id, |t| { t.done = !t.done; })
    }
    pub fn complete(&self, id: i64) -> Result<bool, DbError> {
        self.tasks().update(id, |t| { t.done = true; })
    }
    pub fn uncomplete(&self, id: i64) -> Result<bool, DbError> {
        self.tasks().update(id, |t| { t.done = false; })
    }
    pub fn rename(&self, id: i64, new_title: &str) -> Result<bool, DbError> {
        let t = new_title.trim().to_string();
        if t.is_empty() { return Err(DbError::Validation("title cannot be empty".into())); }
        self.tasks().update(id, move |task| { task.title = t; })
    }
    pub fn remove(&self, id: i64) -> Result<bool, DbError> {
        self.tasks().delete(id)
    }

    // ── READ (flat) ───────────────────────────────────────────────────────────

    pub fn all(&self) -> Vec<Task> {
        self.tasks().query()
            .where_eq("user_id", self.user_id)
            .where_eq("deleted", false)
            .order_by("id", Dir::Asc)
            .fetch().unwrap_or_default()
    }

    pub fn pending(&self) -> Vec<Task> {
        self.tasks().query()
            .where_eq("user_id", self.user_id)
            .where_eq("done",    false)
            .where_eq("deleted", false)
            .order_by("id", Dir::Asc)
            .fetch().unwrap_or_default()
    }

    pub fn completed(&self) -> Vec<Task> {
        self.tasks().query()
            .where_eq("user_id", self.user_id)
            .where_eq("done",    true)
            .where_eq("deleted", false)
            .order_by("updated_at", Dir::Desc)
            .fetch().unwrap_or_default()
    }

    pub fn pending_count(&self) -> i64 {
        self.tasks().query()
            .where_eq("user_id", self.user_id)
            .where_eq("done",    false)
            .where_eq("deleted", false)
            .count().unwrap_or(0)
    }

    pub fn search(&self, q: &str) -> Vec<Task> {
        self.tasks().query()
            .where_eq("user_id",  self.user_id)
            .where_eq("deleted",  false)
            .where_like("title",  format!("%{}%", q.trim()))
            .fetch().unwrap_or_default()
    }

    // ── READ (with FK resolution) ─────────────────────────────────────────────

    /// Fetch all pending tasks together with their Category in ONE extra query.
    ///
    /// Uses find_many() to batch-load categories — no N+1 query problem.
    ///
    /// Returns `(Task, Option<Category>)`:
    ///   - `None` → task has no category or the category was soft-deleted
    ///   - `Some(cat)` → the live category
    pub fn pending_with_category(&self) -> Vec<(Task, Option<Category>)> {
        let tasks = self.pending();
        self.attach_categories(tasks)
    }

    /// All tasks grouped by category name.
    ///
    /// Tasks without a (live) category appear under the key `"Uncategorized"`.
    pub fn pending_by_category(&self) -> HashMap<String, Vec<Task>> {
        // 1. Fetch all live categories for this user
        let cats = self.all_categories();
        let cat_name: HashMap<i64, String> = cats.iter()
            .map(|c| (c.id, c.name.clone()))
            .collect();

        // 2. Fetch all pending tasks
        let tasks = self.pending();

        // 3. Group
        let mut grouped: HashMap<String, Vec<Task>> = HashMap::new();
        for task in tasks {
            let key = task.category_id
                .and_then(|id| cat_name.get(&id))
                .cloned()
                .unwrap_or_else(|| "Uncategorized".into());
            grouped.entry(key).or_default().push(task);
        }
        grouped
    }

    /// Tasks in a specific category (live only).
    pub fn by_category(&self, category_id: i64) -> Vec<Task> {
        self.tasks().query()
            .where_eq("user_id",     self.user_id)
            .where_eq("category_id", category_id)
            .where_eq("deleted",     false)
            .order_by("id", Dir::Asc)
            .fetch().unwrap_or_default()
    }

    /// Tasks that have NO category.
    pub fn uncategorized(&self) -> Vec<Task> {
        self.tasks().query()
            .where_eq("user_id",  self.user_id)
            .where_null("category_id")
            .where_eq("deleted",  false)
            .order_by("id", Dir::Asc)
            .fetch().unwrap_or_default()
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Batch-attach Category objects to a list of tasks.
    /// Fetches all distinct category_ids in ONE SQL query.
    fn attach_categories(&self, tasks: Vec<Task>) -> Vec<(Task, Option<Category>)> {
        // Collect distinct category ids (skip None)
        let cat_ids: Vec<i64> = tasks.iter()
            .filter_map(|t| t.category_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // One batch query — not N individual queries
        let cats = self.categories()
            .find_many(&cat_ids)
            .unwrap_or_default();

        let cat_map: HashMap<i64, Category> = cats.into_iter()
            .filter(|c| !c.deleted)          // exclude soft-deleted categories
            .map(|c| (c.id, c))
            .collect();

        tasks.into_iter()
            .map(|task| {
                let cat = task.category_id.and_then(|id| cat_map.get(&id)).cloned();
                (task, cat)
            })
            .collect()
    }

    // ── JSON export ───────────────────────────────────────────────────────────

    pub fn export_json(&self) -> String {
        let tasks = self.all();
        let cats  = self.all_categories();
        serde_json::to_string_pretty(&serde_json::json!({
            "categories": cats.iter().map(|c| serde_json::json!({
                "id": c.id, "name": c.name, "color": c.color
            })).collect::<Vec<_>>(),
            "tasks": tasks.iter().map(|t| serde_json::json!({
                "id": t.id, "category_id": t.category_id,
                "title": t.title, "done": t.done
            })).collect::<Vec<_>>(),
        })).unwrap_or_default()
    }
}
