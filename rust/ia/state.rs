// src/core/state.rs
//
// AppState now holds ONLY:
//   db:      Db          — the single shared connection handle
//   user_id: i64         — the active user (1 = local default)
//
// No Repository<T> fields — they were removed.
//
// At startup, db.register::<T>() runs CREATE TABLE IF NOT EXISTS for each
// domain. After that, domain repos are built on demand from db.of::<T>()
// inside each repository's own methods.
//
// Key rule for FK: register the parent table BEFORE the child table.
//   categories → tasks   (Category must exist before Task references it)

use db_lib::{Db, DbError};

use crate::todo::category::Category;
use crate::todo::repository::TodoRepository;
use crate::todo::task::Task;
use crate::watch_list::anime::Anime;
use crate::watch_list::repository::WatchListRepository;
use crate::watch_list::scan::Scan;

pub struct AppState {
    /// Shared SQLite connection — cheap to clone (Arc).
    pub db:      Db,
    /// Active user. Always 1 for a local single-user app.
    /// Set to the logged-in user id when multi-user / sync is implemented.
    pub user_id: i64,
}

impl AppState {
    /// Open the database and create all tables.
    ///
    /// Table registration order matters for FK constraints:
    ///   parent tables (no FK) → child tables (has FK to parent)
    pub fn new(db: Db) -> Result<Self, DbError> {
        // ── FK parents first ──────────────────────────────────────────────────
        db.register::<Category>()?;   // no FK deps

        // ── FK children after their parents ──────────────────────────────────
        db.register::<Task>()?;       // FK → categories.id

        // ── Independent tables ────────────────────────────────────────────────
        db.register::<Anime>()?;
        db.register::<Scan>()?;

        Ok(Self { db, user_id: 1 })
    }

    // ── Domain repository accessors ───────────────────────────────────────────
    //
    // Each call returns a new domain repository that clones the Db handle
    // (Arc increment only) and creates sub-repos on demand via db.of::<T>().
    // Calling these every frame is fine.

    /// Task + Category repository for the current user.
    pub fn todo(&self) -> TodoRepository {
        TodoRepository::new(self.db.clone(), self.user_id)
    }

    /// Anime + Scan watchlist for the current user.
    pub fn watchlist(&self) -> WatchListRepository {
        WatchListRepository::new(self.db.clone(), self.user_id)
    }

    // ── Admin (no user_id scope) ──────────────────────────────────────────────
    //
    // Returns raw Repository<T> — no user_id filter is applied automatically.
    // Use only on the admin screen after checking is_admin().

    pub fn admin_tasks(&self)      -> db_lib::Repository<Task>     { self.db.of::<Task>() }
    pub fn admin_categories(&self) -> db_lib::Repository<Category> { self.db.of::<Category>() }
    pub fn admin_anime(&self)      -> db_lib::Repository<Anime>    { self.db.of::<Anime>() }
}
