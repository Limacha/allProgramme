# db_lib — Complete Implementation Tutorial

> **Central idea**: one variable `db: Db` gives you access to every table,
> every query, and every write operation. You never import `rusqlite`.

---

## Table of contents

1. [Project setup](#step-1--project-setup)
2. [Open the database](#step-2--open-the-database)
3. [Define a model](#step-3--define-a-model)
4. [Register the table](#step-4--register-the-table)
5. [Write data through `db`](#step-5--write-data-through-db)
6. [Read data through `db`](#step-6--read-data-through-db)
7. [Update and delete through `db`](#step-7--update-and-delete-through-db)
8. [Foreign keys between models](#step-8--foreign-keys-between-models)
9. [Wrap `db` in a domain repository](#step-9--wrap-db-in-a-domain-repository)
10. [Store `db` in AppState (eframe)](#step-10--store-db-in-appstate-eframe)
11. [Use `db` from the UI](#step-11--use-db-from-the-ui)
12. [Add a new model in 5 steps](#step-12--add-a-new-model-in-5-steps)
13. [Quick reference](#step-13--quick-reference)

---

## Step 1 — Project setup

You need a Cargo workspace so `akTool` can use `db_lib` as a path dependency.

```
my_project/
├── Cargo.toml          ← workspace root
├── db_lib/             ← the generic library (never touch this)
│   ├── Cargo.toml
│   └── src/…
└── akTool/             ← your application
    ├── Cargo.toml
    └── src/…
```

**`Cargo.toml` (workspace root)**
```toml
[workspace]
members  = ["db_lib", "akTool"]
resolver = "2"
```

**`akTool/Cargo.toml`**
```toml
[dependencies]
db_lib     = { path = "../db_lib" }   # ← the only db import you need
eframe     = { version = "0.34.1", features = ["persistence"] }
serde_json = "1"                       # only for JSON export/import
```

> ⚠️ You do **not** add `rusqlite` to `akTool`. The library hides it completely.

---

## Step 2 — Open the database

The entire database lives in one variable: `db`.

```rust
// src/db/mod.rs
use std::path::PathBuf;
pub use db_lib::{Db, DbError};

/// Where the SQLite file lives (platform-specific, handled by eframe).
pub fn db_path() -> PathBuf {
    eframe::storage_dir("myApp")
        .unwrap_or_else(|| PathBuf::from("."))
        .join("app.db")         // → ~/.local/share/myApp/app.db  on Linux
}

/// Open the database. Call this ONCE at startup.
pub fn open() -> Result<Db, DbError> {
    Db::open(&db_path())
    // This sets WAL mode, enables foreign keys, and creates the file if needed.
    // Tables are NOT created here — see Step 4.
}
```

**In your app's startup:**
```rust
let db = crate::db::open()?;
// ↑ This is the ONE variable that controls everything.
```

`db` is an `Arc<Mutex<Connection>>` internally.
**Cloning it is free** (just increments a counter).
You can pass a clone of `db` to every part of your app.

---

## Step 3 — Define a model

A model is a plain Rust struct + an `impl DbRecord` block.
The `impl DbRecord` block is the only place you describe the table structure.

```rust
// src/todo/task.rs

// ① Import only from db_lib — never from rusqlite
use db_lib::{ColType, Column, DbError, DbRecord, IndexDef, SqlValue, ValueSet, now};

// ② Your struct — just a normal Rust struct
#[derive(Clone, Debug)]
pub struct Task {
    pub id:         i64,     // always i64 — SQLite integer PK
    pub user_id:    i64,
    pub title:      String,
    pub done:       bool,
    pub deleted:    bool,    // soft-delete: 1 = removed but kept for sync
    pub updated_at: i64,     // Unix timestamp in seconds
}

impl Task {
    /// Convenience constructor — id starts at 0, the DB assigns the real one.
    pub fn new(user_id: i64, title: &str) -> Self {
        Self {
            id: 0, user_id,
            title:      title.trim().to_string(),
            done:       false,
            deleted:    false,
            updated_at: now(),   // now() is a helper from db_lib (no extra crate)
        }
    }
}

// ③ impl DbRecord — this is the mapping between your struct and the DB table
impl DbRecord for Task {

    // ── Table name ──────────────────────────────────────────────────────────
    // What the SQLite table will be called.
    fn table_name() -> &'static str { "tasks" }

    // ── Column definitions ──────────────────────────────────────────────────
    // Describe every column EXCEPT "id".
    // "id INTEGER PRIMARY KEY AUTOINCREMENT" is ALWAYS added automatically.
    fn columns() -> Vec<Column> {
        vec![
            Column::new("user_id",    ColType::Integer).not_null(),
            Column::new("title",      ColType::Text).not_null(),
            Column::new("done",       ColType::Integer).not_null().default("0"),
            Column::new("deleted",    ColType::Integer).not_null().default("0"),
            Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
        ]
    }

    // ── Indexes (optional) ──────────────────────────────────────────────────
    // The library creates these automatically alongside the table.
    fn indexes() -> Vec<IndexDef> {
        vec![
            IndexDef::new(&["user_id", "deleted"]),  // fast: WHERE user_id = ? AND deleted = 0
        ]
    }

    // ── Row → Struct ────────────────────────────────────────────────────────
    // Called by the library when it reads a row from the DB.
    // Access columns BY NAME — never by index.
    // "id" is always available. Other names must match columns() exactly.
    fn from_values(v: &ValueSet) -> Result<Self, DbError> {
        Ok(Task {
            id:         v.get("id")?.as_i64()?,
            user_id:    v.get("user_id")?.as_i64()?,
            title:      v.get("title")?.as_text()?,
            done:       v.get("done")?.as_bool()?,       // stored as 0/1 in SQLite
            deleted:    v.get("deleted")?.as_bool()?,
            updated_at: v.get("updated_at")?.as_i64()?,
        })
    }

    // ── Struct → Row ────────────────────────────────────────────────────────
    // Called by the library when it writes a row to the DB.
    // Return (column_name, value) pairs.
    // Do NOT include "id" — the DB assigns it.
    fn to_params(&self) -> Vec<(&'static str, SqlValue)> {
        vec![
            ("user_id",    self.user_id.into()),
            ("title",      self.title.clone().into()),
            ("done",       self.done.into()),          // bool → SqlValue::Integer(0 or 1)
            ("deleted",    self.deleted.into()),
            ("updated_at", now().into()),               // always stamp the current time
        ]
    }

    // ── Primary key ─────────────────────────────────────────────────────────
    // Return None if the record has not been inserted yet (id == 0).
    fn id(&self)               -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
    fn set_id(&mut self, id: i64)              { self.id = id; }
    //         ↑ Called by the library after INSERT to give you the real id back
}
```

### Type mapping cheat-sheet

| Rust type | ColType | Default example | Read back with |
|---|---|---|---|
| `i64` / `i32` / `u32` | `Integer` | `"0"` | `.as_i64()?` |
| `bool` | `Integer` | `"0"` | `.as_bool()?` (0=false, else=true) |
| `f64` / `f32` | `Real` | `"0.0"` | `.as_f64()?` |
| `String` | `Text` | `"'hello'"` | `.as_text()?` |
| `Option<String>` | `Text` | *(none)* | `.as_opt_text()?` |
| `Option<i64>` | `Integer` | *(none)* | `.as_opt_i64()?` |
| Unix timestamp | `Integer` | `"(unixepoch())"` | `.as_i64()?` |
| Enum as text | `Text` | `"'MyVariant'"` | `.as_text()? → From<&str>` |

---

## Step 4 — Register the table

`db.register::<T>()` creates the table in SQLite **once** at startup.
Calling it again is a no-op (`CREATE TABLE IF NOT EXISTS`).

```rust
// In your startup code (e.g., AppState::new or App::new):

let db = crate::db::open()?;

// Register every model you use.
// The table is created here — you only pay this cost ONCE.
db.register::<Task>()?;

// ✅ After this, db.of::<Task>() is free (no SQL, no allocation).
```

> **FK rule**: always register the **parent** table before the **child** table.
> If `Task` has a FK to `Category`, do:
> ```rust
> db.register::<Category>()?;   // parent first
> db.register::<Task>()?;       // child second
> ```

---

## Step 5 — Write data through `db`

Once registered, you get a `Repository<T>` from `db` using `db.of::<T>()`.
This is the variable you use to write.

```rust
// Get a repository for Task — CHEAP (just an Arc clone, no SQL)
let tasks = db.of::<Task>();

// INSERT
let mut new_task = Task::new(1, "Buy milk");
let id = tasks.insert(new_task)?;
// ↑ Returns the new row's id (i64).
// The library ran:
//   INSERT INTO "tasks" ("user_id", "title", "done", "deleted", "updated_at")
//   VALUES (?, ?, ?, ?, ?)
// All values are parameterized — no SQL injection possible.

println!("Inserted task with id {id}");
```

`db.of::<Task>()` is so cheap you can call it **every frame** in a game loop
or every call in a UI method — it does no SQL.

---

## Step 6 — Read data through `db`

```rust
let tasks = db.of::<Task>();

// ── Find one row by primary key ──────────────────────────────────────────────
let maybe_task: Option<Task> = tasks.find(42)?;

// ── Build a query (chainable) ────────────────────────────────────────────────
let pending: Vec<Task> = tasks
    .query()
    .where_eq("user_id",  1i64)
    .where_eq("done",     false)
    .where_eq("deleted",  false)
    .order_by("id", Dir::Asc)
    .fetch()?;

// ── Count ─────────────────────────────────────────────────────────────────────
let n: i64 = tasks
    .query()
    .where_eq("user_id", 1i64)
    .where_eq("done",    false)
    .where_eq("deleted", false)
    .count()?;

// ── Fetch first match ─────────────────────────────────────────────────────────
let first: Option<Task> = tasks
    .query()
    .where_eq("user_id", 1i64)
    .order_by("updated_at", Dir::Desc)
    .fetch_one()?;

// ── Search by substring ───────────────────────────────────────────────────────
let results: Vec<Task> = tasks
    .query()
    .where_eq("user_id",  1i64)
    .where_eq("deleted",  false)
    .where_like("title",  "%milk%".to_string())   // % is the wildcard
    .fetch()?;

// ── Paginate ──────────────────────────────────────────────────────────────────
let page = 0i64;
let page_size = 20i64;
let page_data: Vec<Task> = tasks
    .query()
    .where_eq("user_id", 1i64)
    .where_eq("deleted", false)
    .order_by("id", Dir::Asc)
    .limit(page_size)
    .offset(page * page_size)
    .fetch()?;

// ── Fetch batch by ids ────────────────────────────────────────────────────────
// (Useful for FK resolution — see Step 8)
let ids = vec![1i64, 5, 9, 22];
let batch: Vec<Task> = tasks.find_many(&ids)?;
// Runs: SELECT … WHERE id IN (?, ?, ?, ?)  — one query, not four
```

### All filter methods

```rust
.where_eq("col",      value)       // col = ?
.where_neq("col",     value)       // col != ?
.where_gt("col",      value)       // col > ?
.where_gte("col",     value)       // col >= ?
.where_lt("col",      value)       // col < ?
.where_lte("col",     value)       // col <= ?
.where_like("col",    "%pattern%") // col LIKE ?
.where_null("col")                 // col IS NULL
.where_not_null("col")             // col IS NOT NULL
.where_in("col", vec![1i64, 2, 3]) // col IN (?, ?, ?)
```

---

## Step 7 — Update and delete through `db`

```rust
let tasks = db.of::<Task>();

// ── UPDATE via closure ────────────────────────────────────────────────────────
// The library fetches the row, runs your closure, then writes it back.
// Returns Ok(true) if found, Ok(false) if the id does not exist.
tasks.update(42, |t| {
    t.done  = true;
    t.title = "Buy oat milk".into();
})?;

// ── SOFT DELETE ───────────────────────────────────────────────────────────────
// Sets deleted = 1. The row stays in the DB as a tombstone for future sync.
// Normal queries use .where_eq("deleted", false) to skip these rows.
tasks.delete(42)?;

// ── HARD DELETE ───────────────────────────────────────────────────────────────
// Permanently removes the row. Use only when you are sure you never need
// to sync this deletion to another device.
tasks.delete_hard(42)?;

// ── INSERT MANY (in one transaction) ─────────────────────────────────────────
let new_tasks = vec![
    Task::new(1, "Task A"),
    Task::new(1, "Task B"),
    Task::new(1, "Task C"),
];
let ids: Vec<i64> = tasks.insert_many(new_tasks)?;
```

---

## Step 8 — Foreign keys between models

### Define the FK in the child model

```rust
// src/todo/task.rs

use db_lib::{OnDelete, …};   // ← import OnDelete

impl DbRecord for Task {
    fn columns() -> Vec<Column> {
        vec![
            Column::new("user_id",     ColType::Integer).not_null(),

            // ↓ FK to categories table
            Column::new("category_id", ColType::Integer)
                .references("categories", "id")  // REFERENCES "categories"("id")
                .on_delete(OnDelete::SetNull),    // ON DELETE SET NULL
                // When the parent Category row is hard-deleted,
                // SQLite automatically sets category_id to NULL here.

            Column::new("title",       ColType::Text).not_null(),
            Column::new("done",        ColType::Integer).not_null().default("0"),
            Column::new("deleted",     ColType::Integer).not_null().default("0"),
            Column::new("updated_at",  ColType::Integer).not_null().default("(unixepoch())"),
        ]
    }
    …
}
```

And the Rust struct field:
```rust
pub struct Task {
    pub id:          i64,
    pub user_id:     i64,
    pub category_id: Option<i64>,   // None = uncategorized
    …
}
```

In `from_values`:
```rust
category_id: v.get("category_id")?.as_opt_i64()?,
```

In `to_params`:
```rust
("category_id", self.category_id.into()),  // None → NULL in SQLite
```

### Register order

```rust
db.register::<Category>()?;  // ← parent first (no FK)
db.register::<Task>()?;      // ← child second (has FK to categories)
```

### Resolve FK: fetch tasks with their categories (no N+1)

```rust
// Step 1 — fetch tasks
let tasks: Vec<Task> = db.of::<Task>()
    .query()
    .where_eq("user_id", 1i64)
    .where_eq("deleted", false)
    .fetch()?;

// Step 2 — collect distinct category ids
let cat_ids: Vec<i64> = tasks.iter()
    .filter_map(|t| t.category_id)
    .collect::<std::collections::HashSet<_>>()
    .into_iter()
    .collect();

// Step 3 — one batch query (NOT one query per task)
let categories: Vec<Category> = db.of::<Category>()
    .find_many(&cat_ids)?;
//  ↑ Runs: SELECT … FROM categories WHERE id IN (?, ?, …)
//    One SQL query for all categories — no N+1 problem.

// Step 4 — build a lookup map and join
let cat_map: HashMap<i64, Category> = categories.into_iter()
    .map(|c| (c.id, c))
    .collect();

let result: Vec<(Task, Option<Category>)> = tasks.into_iter()
    .map(|task| {
        let cat = task.category_id.and_then(|id| cat_map.get(&id)).cloned();
        (task, cat)
    })
    .collect();
```

### OnDelete options

| Option | What happens to Task when Category is deleted |
|---|---|
| `OnDelete::Cascade` | Task is also deleted |
| `OnDelete::SetNull` | Task's `category_id` becomes NULL |
| `OnDelete::Restrict` | The deletion of Category is rejected if tasks exist |
| `OnDelete::NoAction` | Same as Restrict, checked at transaction end |

---

## Step 9 — Wrap `db` in a domain repository

For production code, don't call `db.of::<T>()` directly in your UI.
Instead, build a small **domain repository** that stores `db` and exposes
intention-revealing methods.

```rust
// src/todo/repository.rs

use db_lib::{Db, DbError, Dir};
use super::task::Task;
use super::category::Category;

pub struct TodoRepository {
    db:      Db,          // ← the same db variable, just cloned (free)
    user_id: i64,
}

impl TodoRepository {
    pub fn new(db: Db, user_id: i64) -> Self {
        Self { db, user_id }
    }

    // ── Private sub-repos (created on demand, zero cost) ─────────────────────
    fn tasks(&self)      -> db_lib::Repository<Task>     { self.db.of::<Task>() }
    fn categories(&self) -> db_lib::Repository<Category> { self.db.of::<Category>() }

    // ── Domain methods (what the UI calls) ───────────────────────────────────

    pub fn add(&self, title: &str) -> Result<i64, DbError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(DbError::Validation("title cannot be empty".into()));
        }
        self.tasks().insert(Task::new(self.user_id, title))
    }

    pub fn toggle(&self, id: i64) -> Result<bool, DbError> {
        self.tasks().update(id, |t| { t.done = !t.done; })
    }

    pub fn remove(&self, id: i64) -> Result<bool, DbError> {
        self.tasks().delete(id)
    }

    pub fn pending(&self) -> Vec<Task> {
        self.tasks().query()
            .where_eq("user_id", self.user_id)
            .where_eq("done",    false)
            .where_eq("deleted", false)
            .order_by("id", Dir::Asc)
            .fetch()
            .unwrap_or_default()
    }

    pub fn pending_count(&self) -> i64 {
        self.tasks().query()
            .where_eq("user_id", self.user_id)
            .where_eq("done",    false)
            .where_eq("deleted", false)
            .count()
            .unwrap_or(0)
    }
}
```

The UI now calls `repo.add("Buy milk")` instead of raw DB code.

---

## Step 10 — Store `db` in AppState (eframe)

`AppState` holds **only** `db` and `user_id`.
No `Repository<T>` fields — they are created on demand.

```rust
// src/core/state.rs

use db_lib::Db;
use crate::todo::task::Task;
use crate::todo::category::Category;
use crate::todo::repository::TodoRepository;

pub struct AppState {
    pub db:      Db,       // ← the one variable that controls everything
    pub user_id: i64,
}

impl AppState {
    pub fn new(db: Db) -> Result<Self, db_lib::DbError> {
        // Register tables — CREATE TABLE IF NOT EXISTS, runs once
        // Parent tables BEFORE child tables that FK to them
        db.register::<Category>()?;   // parent
        db.register::<Task>()?;       // child (FK → categories.id)

        Ok(Self { db, user_id: 1 })
    }

    // ── Accessors — each returns a domain repository ─────────────────────────
    // Calling these every frame is FINE (only clones the Arc pointer)

    pub fn todo(&self) -> TodoRepository {
        TodoRepository::new(self.db.clone(), self.user_id)
    }

    // ── Admin access — no user_id filter ─────────────────────────────────────
    pub fn all_tasks_admin(&self) -> db_lib::Repository<Task> {
        self.db.of::<Task>()
    }
}
```

**Opening the database and building AppState:**
```rust
// In App::new (eframe):
let db    = crate::db::open().expect("cannot open database");
let state = AppState::new(db).expect("cannot initialize tables");
```

---

## Step 11 — Use `db` from the UI

In `eframe::App::update()`, the pattern is always:

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    // 1. READ — call domain methods to get data for rendering
    let pending = self.state.todo().pending();
    let count   = self.state.todo().pending_count();

    // 2. COLLECT mutations (don't apply yet — Rust borrow rules)
    let mut toggle_id: Option<i64> = None;
    let mut delete_id: Option<i64> = None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(format!("{count} tasks pending"));

        for task in &pending {
            ui.horizontal(|ui| {
                if ui.button("☐").clicked() { toggle_id = Some(task.id); }
                ui.label(&task.title);
                if ui.small_button("🗑").clicked() { delete_id = Some(task.id); }
            });
        }
    });

    // 3. APPLY mutations AFTER the UI borrow ends
    if let Some(id) = toggle_id {
        if let Err(e) = self.state.todo().toggle(id) {
            eprintln!("error: {e}");
        }
    }
    if let Some(id) = delete_id {
        self.state.todo().remove(id).ok();
    }
}
```

> **Why collect then apply?**
> `self.state.todo().pending()` borrows `self.state` immutably.
> `self.state.todo().toggle(id)` needs a mutable borrow.
> You cannot have both at the same time in Rust.
> The fix: store the id while rendering, then call the mutation after the loop.

---

## Step 12 — Add a new model in 5 steps

Here is the complete checklist for adding a **Notes** feature.

### ① Create the model file

`src/notes/note.rs`

```rust
use db_lib::{ColType, Column, DbError, DbRecord, IndexDef, SqlValue, ValueSet, now};

#[derive(Clone, Debug)]
pub struct Note {
    pub id:         i64,
    pub user_id:    i64,
    pub content:    String,
    pub pinned:     bool,
    pub deleted:    bool,
    pub updated_at: i64,
}

impl Note {
    pub fn new(user_id: i64, content: &str) -> Self {
        Self { id: 0, user_id, content: content.into(),
               pinned: false, deleted: false, updated_at: now() }
    }
}

impl DbRecord for Note {
    fn table_name() -> &'static str { "notes" }

    fn columns() -> Vec<Column> { vec![
        Column::new("user_id",    ColType::Integer).not_null(),
        Column::new("content",    ColType::Text).not_null(),
        Column::new("pinned",     ColType::Integer).not_null().default("0"),
        Column::new("deleted",    ColType::Integer).not_null().default("0"),
        Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
    ]}

    fn indexes() -> Vec<IndexDef> { vec![
        IndexDef::new(&["user_id", "deleted", "pinned"]),
    ]}

    fn from_values(v: &ValueSet) -> Result<Self, DbError> {
        Ok(Note {
            id:         v.get("id")?.as_i64()?,
            user_id:    v.get("user_id")?.as_i64()?,
            content:    v.get("content")?.as_text()?,
            pinned:     v.get("pinned")?.as_bool()?,
            deleted:    v.get("deleted")?.as_bool()?,
            updated_at: v.get("updated_at")?.as_i64()?,
        })
    }

    fn to_params(&self) -> Vec<(&'static str, SqlValue)> { vec![
        ("user_id",    self.user_id.into()),
        ("content",    self.content.clone().into()),
        ("pinned",     self.pinned.into()),
        ("deleted",    self.deleted.into()),
        ("updated_at", now().into()),
    ]}

    fn id(&self)               -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
    fn set_id(&mut self, id: i64)              { self.id = id; }
}
```

### ② Create the domain repository

`src/notes/repository.rs`

```rust
use db_lib::{Db, DbError, Dir};
use super::note::Note;

pub struct NotesRepository { db: Db, user_id: i64 }

impl NotesRepository {
    pub fn new(db: Db, user_id: i64) -> Self { Self { db, user_id } }
    fn notes(&self) -> db_lib::Repository<Note> { self.db.of::<Note>() }

    pub fn add(&self, content: &str) -> Result<i64, DbError> {
        self.notes().insert(Note::new(self.user_id, content))
    }
    pub fn pin(&self, id: i64) -> Result<bool, DbError> {
        self.notes().update(id, |n| { n.pinned = true; })
    }
    pub fn remove(&self, id: i64) -> Result<bool, DbError> {
        self.notes().delete(id)
    }
    pub fn all(&self) -> Vec<Note> {
        self.notes().query()
            .where_eq("user_id", self.user_id)
            .where_eq("deleted", false)
            .order_by("pinned",     Dir::Desc)
            .order_by("updated_at", Dir::Desc)
            .fetch().unwrap_or_default()
    }
}
```

### ③ Create `mod.rs`

`src/notes/mod.rs`
```rust
pub mod note;
pub mod repository;
```

### ④ Register in `AppState::new`

```rust
// src/core/state.rs
use crate::notes::note::Note;
use crate::notes::repository::NotesRepository;

impl AppState {
    pub fn new(db: Db) -> Result<Self, db_lib::DbError> {
        db.register::<Category>()?;
        db.register::<Task>()?;
        db.register::<Note>()?;    // ← add this line
        Ok(Self { db, user_id: 1 })
    }

    pub fn notes(&self) -> NotesRepository {              // ← add this method
        NotesRepository::new(self.db.clone(), self.user_id)
    }
}
```

### ⑤ Declare the module in `lib.rs`

```rust
// src/lib.rs
pub mod notes;    // ← add this line
```

**Done.** The `notes` table is created on next launch. No migration script needed.

---

## Step 13 — Quick reference

### The `db` variable does everything

```rust
// Open once
let db = Db::open(&path)?;

// Register tables (once at startup)
db.register::<MyModel>()?;

// Get a repository (cheap, any time)
let repo = db.of::<MyModel>();

// Or combined (register + get):
let repo = db.repository::<MyModel>()?;
```

### All operations through `repo`

```rust
let repo = db.of::<MyModel>();

// Write
repo.insert(record)?                       // INSERT, returns new id
repo.update(id, |r| { r.field = val; })?  // UPDATE via closure
repo.delete(id)?                           // soft delete (deleted = 1)
repo.delete_hard(id)?                      // hard DELETE
repo.insert_many(records)?                 // batch INSERT in one transaction

// Read
repo.find(id)?                             // Option<T>
repo.find_many(&[id1, id2])?               // Vec<T>  — one SQL query

// Query
repo.query()
    .where_eq("col",   value)
    .where_neq("col",  value)
    .where_gt("col",   value)
    .where_gte("col",  value)
    .where_lt("col",   value)
    .where_lte("col",  value)
    .where_like("col", "%pattern%")
    .where_null("col")
    .where_not_null("col")
    .where_in("col",   vec![1i64, 2, 3])
    .order_by("col", Dir::Asc)
    .order_by("col", Dir::Desc)
    .limit(20)
    .offset(page * 20)
    .fetch()?       // Vec<T>
    .fetch_one()?   // Option<T>
    .count()?       // i64
    .exists()?      // bool
```

### Column builder

```rust
Column::new("name", ColType::Integer)   // basic
    .not_null()                          // NOT NULL
    .default("0")                        // DEFAULT 0
    .references("other_table", "id")     // FK REFERENCES "other_table"("id")
    .on_delete(OnDelete::Cascade)        // ON DELETE CASCADE
    .on_delete(OnDelete::SetNull)        // ON DELETE SET NULL
    .on_delete(OnDelete::Restrict)       // ON DELETE RESTRICT
```

### FK registration order

```rust
// Parent before child — ALWAYS
db.register::<ParentModel>()?;   // no FK
db.register::<ChildModel>()?;    // FK → parent
```

### What to import in the app (never rusqlite)

```rust
// In a model file:
use db_lib::{ColType, Column, DbError, DbRecord, IndexDef, OnDelete, SqlValue, ValueSet, now};

// In a repository file:
use db_lib::{Db, DbError, Dir};

// In state.rs / app.rs:
use db_lib::{Db, DbError};
```

### Checklist for a new domain

```
□  src/<domain>/model.rs        — struct + impl DbRecord
□  src/<domain>/repository.rs   — domain methods (stores Db, calls db.of::<T>())
□  src/<domain>/mod.rs          — pub mod model; pub mod repository;
□  src/lib.rs                   — pub mod <domain>;
□  src/core/state.rs            — db.register::<Model>()? in new()
                                — pub fn <domain>(&self) -> Repository accessor
□  src/app/app.rs               — self.state.<domain>().method() in update()
```
