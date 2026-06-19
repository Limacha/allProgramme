# Tutorial — Implementing `db_lib` in an Application

> **Goal**: take the generic SQLite library (`db_lib`) and wire it into a new
> feature from scratch, without ever importing `rusqlite`.

---

## Table of contents

1. [Project setup](#1-project-setup)
2. [Define a model](#2-define-a-model)
3. [Open the database](#3-open-the-database)
4. [Generic repository vs domain repository](#4-generic-repository-vs-domain-repository)
5. [CRUD operations](#5-crud-operations)
6. [Querying with QueryBuilder](#6-querying-with-querybuilder)
7. [Wire into AppState](#7-wire-into-appstate)
8. [Use in the UI (eframe)](#8-use-in-the-ui-eframe)
9. [JSON export and import](#9-json-export-and-import)
10. [Add indexes for performance](#10-add-indexes-for-performance)
11. [Admin — queries without user scope](#11-admin--queries-without-user-scope)
12. [Add a brand-new domain (full walkthrough)](#12-add-a-brand-new-domain-full-walkthrough)
13. [Cheat sheet](#13-cheat-sheet)

---

## 1. Project setup

Your workspace must declare both crates:

```toml
# /Cargo.toml  (workspace root)
[workspace]
members  = ["db_lib", "akTool"]
resolver = "2"
```

In your application's `Cargo.toml`, add `db_lib` as a path dependency.
**You do NOT add `rusqlite` — the lib hides it entirely.**

```toml
# akTool/Cargo.toml
[dependencies]
db_lib     = { path = "../db_lib" }
eframe     = { version = "0.34.1", features = ["persistence"] }
serde_json = "1"          # only for JSON export/import screens
```

In **every** application file that touches the database, the only import is:

```rust
use db_lib::{
    Db,          // connection handle
    Repository,  // generic CRUD
    DbRecord,    // trait your structs implement
    Column,      // column definition
    ColType,     // INTEGER / REAL / TEXT
    IndexDef,    // optional composite indexes
    SqlValue,    // the value type (replaces rusqlite types)
    ValueSet,    // row handle passed to from_values()
    DbError,     // the only error type
    QueryBuilder,// chainable queries
    Dir,         // Asc / Desc
    now,         // current Unix timestamp helper
};
```

---

## 2. Define a model

Every struct that you want to store needs four things:

| What | Why |
|---|---|
| Plain fields (id, user_id, …) | The data you care about |
| `impl DbRecord` | Tells the lib how to create the table and map rows |
| `columns()` | Declares the SQL schema (NOT including `id`) |
| `from_values()` + `to_params()` | Serialize / deserialize one row |

### Minimal example — `Book`

```rust
// src/reading/book.rs

use db_lib::{ColType, Column, DbError, DbRecord, SqlValue, ValueSet, now};

#[derive(Clone, Debug)]
pub struct Book {
    pub id:         i64,    // 0 = not yet inserted
    pub user_id:    i64,
    pub title:      String,
    pub author:     String,
    pub pages:      i64,
    pub read:       bool,
    pub deleted:    bool,
    pub updated_at: i64,
}

impl Book {
    /// Helper constructor — id starts at 0, the DB assigns the real one.
    pub fn new(user_id: i64, title: &str, author: &str, pages: i64) -> Self {
        Self {
            id: 0, user_id,
            title:      title.into(),
            author:     author.into(),
            pages,
            read:       false,
            deleted:    false,
            updated_at: now(),
        }
    }
}

impl DbRecord for Book {
    // ── 1. Table name ─────────────────────────────────────────────────────────
    fn table_name() -> &'static str { "books" }

    // ── 2. Schema (every column EXCEPT id) ───────────────────────────────────
    //
    // Rules:
    //  • Column::new("name", ColType) creates a column
    //  • .not_null()     adds NOT NULL constraint
    //  • .default("0")   adds DEFAULT 0 (any SQL expression)
    //  • id is ALWAYS added automatically — never put it here
    fn columns() -> Vec<Column> {
        vec![
            Column::new("user_id",    ColType::Integer).not_null(),
            Column::new("title",      ColType::Text).not_null(),
            Column::new("author",     ColType::Text).not_null(),
            Column::new("pages",      ColType::Integer).not_null().default("0"),
            Column::new("read",       ColType::Integer).not_null().default("0"),
            Column::new("deleted",    ColType::Integer).not_null().default("0"),
            Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
        ]
    }

    // ── 3. Row → struct ───────────────────────────────────────────────────────
    //
    // Access every column BY NAME — never by index.
    // "id" is always available. Other names must match columns() exactly.
    fn from_values(v: &ValueSet) -> Result<Self, DbError> {
        Ok(Book {
            id:         v.get("id")?.as_i64()?,
            user_id:    v.get("user_id")?.as_i64()?,
            title:      v.get("title")?.as_text()?,
            author:     v.get("author")?.as_text()?,
            pages:      v.get("pages")?.as_i64()?,
            read:       v.get("read")?.as_bool()?,
            deleted:    v.get("deleted")?.as_bool()?,
            updated_at: v.get("updated_at")?.as_i64()?,
        })
    }

    // ── 4. struct → row ───────────────────────────────────────────────────────
    //
    // Return the VALUES to write. Must match the columns() list.
    // Do NOT include "id" — the DB assigns it.
    // Always set updated_at to now() on every write.
    fn to_params(&self) -> Vec<(&'static str, SqlValue)> {
        vec![
            ("user_id",    self.user_id.into()),
            ("title",      self.title.clone().into()),
            ("author",     self.author.clone().into()),
            ("pages",      self.pages.into()),
            ("read",       self.read.into()),
            ("deleted",    self.deleted.into()),
            ("updated_at", now().into()),
        ]
    }

    // ── 5. Primary key helpers ────────────────────────────────────────────────
    fn id(&self)               -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
    fn set_id(&mut self, id: i64)              { self.id = id; }
}
```

### Type mapping reference

| Rust type | Column type | SqlValue | Read back with |
|---|---|---|---|
| `i64`, `i32`, `u32` | `ColType::Integer` | `.into()` | `.as_i64()?` |
| `bool` | `ColType::Integer` + `.default("0")` | `.into()` | `.as_bool()?` |
| `f64`, `f32` | `ColType::Real` | `.into()` | `.as_f64()?` |
| `String` | `ColType::Text` | `.clone().into()` | `.as_text()?` |
| `&str` | `ColType::Text` | `.into()` | `.as_text()?` |
| `Option<String>` | `ColType::Text` | `.into()` | `.as_opt_text()?` |
| Unix timestamp | `ColType::Integer` + `.default("(unixepoch())")` | `now().into()` | `.as_i64()?` |
| Enum stored as text | `ColType::Text` + `.default("'Variant'")` | `self.status.as_str().into()` | parse with `From<&str>` |

---

## 3. Open the database

Always open the database **once** at startup. The `Db` handle is an
`Arc<Mutex<Connection>>` — cloning it is free.

```rust
// src/db/mod.rs

use std::path::PathBuf;
pub use db_lib::{Db, DbError, Dir, QueryBuilder, Repository, SqlValue};

/// Returns the OS-appropriate path for the SQLite file.
pub fn db_path() -> PathBuf {
    eframe::storage_dir("akTool")           // Linux: ~/.local/share/akTool/
        .unwrap_or_else(|| PathBuf::from("."))
        .join("app.db")
}

/// Open the database (WAL mode, foreign keys ON, tables created on first run).
pub fn open() -> Result<Db, DbError> {
    Db::open(&db_path())
}
```

In your eframe `App::new`:
```rust
let db = crate::db::open().expect("failed to open database");
```

---

## 4. Generic repository vs domain repository

### Generic `Repository<T>` (from the lib)

The library provides `Repository<T>` which can do everything generically.
You get one by calling `db.repository::<T>()`:

```rust
let book_repo: Repository<Book> = db.repository::<Book>()?;
// ↑ This also runs CREATE TABLE IF NOT EXISTS books (…) automatically.
```

You can use it directly for simple cases:

```rust
// Insert
let id = book_repo.insert(Book::new(1, "Dune", "Herbert", 412))?;

// Find by id
let book: Option<Book> = book_repo.find(id)?;

// Update via closure — read-modify-write in one call
book_repo.update(id, |b| { b.read = true; })?;

// Soft delete (sets deleted = 1)
book_repo.delete(id)?;

// Raw query
let results: Vec<Book> = book_repo.query()
    .where_eq("user_id", 1i64)
    .where_eq("deleted", false)
    .order_by("title", Dir::Asc)
    .fetch()?;
```

### Domain `ReadingRepository` (your code)

For production use, **wrap** `Repository<T>` in a domain repository.
This gives your UI clean, intention-revealing methods.

```rust
// src/reading/repository.rs

use db_lib::{DbError, Dir, Repository};
use super::book::Book;

pub struct ReadingRepository {
    repo:    Repository<Book>,
    user_id: i64,
}

impl ReadingRepository {
    pub fn new(repo: Repository<Book>, user_id: i64) -> Self {
        Self { repo, user_id }
    }

    pub fn add(&self, title: &str, author: &str, pages: i64) -> Result<i64, DbError> {
        self.repo.insert(Book::new(self.user_id, title, author, pages))
    }

    pub fn mark_read(&self, id: i64) -> Result<bool, DbError> {
        self.repo.update(id, |b| { b.read = true; })
    }

    pub fn all_unread(&self) -> Vec<Book> {
        self.repo.query()
            .where_eq("user_id", self.user_id)
            .where_eq("read",    false)
            .where_eq("deleted", false)
            .order_by("title", Dir::Asc)
            .fetch()
            .unwrap_or_default()
    }

    // … more domain methods
}
```

**Rule of thumb**: use `Repository<T>` directly only in `AppState` and admin code.
The UI always goes through a domain repository.

---

## 5. CRUD operations

### INSERT

```rust
// Create the struct (id = 0)
let mut book = Book::new(user_id, "The Hobbit", "Tolkien", 310);

// Insert returns the new database id
let id: i64 = repo.insert(book)?;
// book.id is still 0 — if you need the id back, use the return value
```

> **Why `mut`?** `insert` calls `set_id` on your struct after the INSERT.
> If you don't need the id on the struct, pass by value and ignore.

### UPDATE

`update(id, closure)` fetches the row, runs your closure on it, then writes it back:

```rust
// Rename a book
repo.update(42, |b| {
    b.title = "The Hobbit (Revised)".into();
})?;

// Mark as read and update author simultaneously
repo.update(42, |b| {
    b.read   = true;
    b.author = "J.R.R. Tolkien".into();
})?;
```

Returns `Ok(true)` if found, `Ok(false)` if the id does not exist.

### SOFT DELETE (recommended)

Keeps a tombstone for future sync. The row stays in the DB with `deleted = 1`.

```rust
repo.delete(42)?;   // sets deleted = 1, updates updated_at
```

All normal queries use `.where_eq("deleted", false)` to exclude them.

### HARD DELETE (permanent)

```rust
repo.delete_hard(42)?;  // removes the row entirely
```

⚠️ Use only when you are sure you will never sync to another device.

### FIND by id

```rust
match repo.find(42)? {
    Some(book) => println!("Found: {}", book.title),
    None       => println!("Not found"),
}
```

---

## 6. Querying with QueryBuilder

`repo.query()` returns a `QueryBuilder<T>`. Chain any number of methods,
then call a **terminal** (`.fetch()`, `.count()`, `.fetch_one()`, `.exists()`).

```rust
let results: Vec<Book> = repo.query()
    .where_eq("user_id",  1i64)
    .where_eq("deleted",  false)
    .where_eq("read",     false)
    .order_by("updated_at", Dir::Desc)
    .limit(20)
    .offset(0)
    .fetch()?;
```

### All filter methods

```rust
.where_eq("col",  value)    // col = ?
.where_neq("col", value)    // col != ?
.where_gt("col",  value)    // col > ?
.where_gte("col", value)    // col >= ?
.where_lt("col",  value)    // col < ?
.where_lte("col", value)    // col <= ?
.where_like("col", "%txt%") // col LIKE ? (add % yourself)
.where_null("col")          // col IS NULL
.where_not_null("col")      // col IS NOT NULL
```

### Terminal methods

```rust
.fetch()?          // → Vec<T>
.fetch_one()?      // → Option<T>  (adds LIMIT 1 internally)
.count()?          // → i64
.exists()?         // → bool
```

### Practical patterns

```rust
// Search by title substring
repo.query()
    .where_eq("user_id", user_id)
    .where_eq("deleted",  false)
    .where_like("title",  format!("%{}%", search_term))
    .fetch()?;

// Most recently modified first
repo.query()
    .where_eq("user_id", user_id)
    .where_eq("deleted",  false)
    .order_by("updated_at", Dir::Desc)
    .limit(10)
    .fetch()?;

// Count unread books
let n: i64 = repo.query()
    .where_eq("user_id", user_id)
    .where_eq("read",    false)
    .where_eq("deleted", false)
    .count()?;

// Does this user have any books?
let has_books: bool = repo.query()
    .where_eq("user_id", user_id)
    .where_eq("deleted",  false)
    .exists()?;
```

---

## 7. Wire into AppState

`AppState` owns one `Repository<T>` per domain. The table is created
the first time `db.repository::<T>()` is called.

```rust
// src/core/state.rs

use db_lib::{Db, DbError, Repository};
use crate::reading::book::Book;
use crate::reading::repository::ReadingRepository;
// … other imports

pub struct AppState {
    pub db:      Db,
    pub user_id: i64,

    // One handle per domain — table created at construction
    task_repo:    Repository<Task>,
    anime_repo:   Repository<Anime>,
    scan_repo:    Repository<Scan>,
    book_repo:    Repository<Book>,   // ← new domain
}

impl AppState {
    pub fn new(db: Db) -> Result<Self, DbError> {
        Ok(Self {
            task_repo:  db.repository::<Task>()?,
            anime_repo: db.repository::<Anime>()?,
            scan_repo:  db.repository::<Scan>()?,
            book_repo:  db.repository::<Book>()?,  // ← new domain
            user_id: 1,
            db,
        })
    }

    pub fn todo(&self)     -> TodoRepository     { TodoRepository::new(self.task_repo.clone(), self.user_id) }
    pub fn watchlist(&self)-> WatchListRepository { WatchListRepository::new(self.anime_repo.clone(), self.scan_repo.clone(), self.user_id) }
    pub fn reading(&self)  -> ReadingRepository  { ReadingRepository::new(self.book_repo.clone(), self.user_id) }  // ← new
}
```

> **Why clone the repository?**
> `Repository<T>` is `Arc<Mutex<Connection>>` + metadata — cloning it is
> just an atomic reference count increment (~nanoseconds). It is safe and
> encouraged.

---

## 8. Use in the UI (eframe)

In `App::update()` you call the domain repository every frame.
No SQL anywhere near the UI.

```rust
// src/app/app.rs  (simplified)

fn ui_reading(&mut self, ui: &mut egui::Ui) {
    ui.heading("Reading List");

    // ── Add book ──────────────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut self.book_title_input);
        if ui.button("Add").clicked() && !self.book_title_input.is_empty() {
            match self.state.reading().add(
                &self.book_title_input, "Unknown", 0
            ) {
                Ok(id)  => { self.status = format!("Book #{id} added"); }
                Err(e)  => { self.status = format!("Error: {e}"); }
            }
            self.book_title_input.clear();
        }
    });

    ui.separator();

    // ── List unread books ─────────────────────────────────────────────────────
    let mut mark_read_id: Option<i64> = None;
    let mut delete_id:    Option<i64> = None;

    for book in self.state.reading().all_unread() {
        ui.horizontal(|ui| {
            ui.label(&book.title);
            ui.label(format!("— {}", book.author));
            if ui.button("✔ Read").clicked()  { mark_read_id = Some(book.id); }
            if ui.small_button("🗑").clicked() { delete_id    = Some(book.id); }
        });
    }

    // Apply mutations AFTER the borrow of self.state ends (Rust borrow rules)
    if let Some(id) = mark_read_id {
        if let Err(e) = self.state.reading().mark_read(id) {
            self.status = e.to_string();
        }
    }
    if let Some(id) = delete_id {
        if let Err(e) = self.state.book_repo_direct().delete(id) {
            self.status = e.to_string();
        }
    }
}
```

### Important egui pattern: collect mutations before applying

Because `self.state.reading().all_unread()` borrows `self.state`, you
**cannot** call `self.state.reading().mark_read(id)` inside the same loop.
The pattern is always:

```rust
let mut action_id: Option<i64> = None;

for item in self.state.reading().all_unread() {
    if ui.button("Mark read").clicked() {
        action_id = Some(item.id);   // ← collect the id
    }
}

// ← all borrows from the loop are gone here
if let Some(id) = action_id {
    self.state.reading().mark_read(id).ok();  // ← now safe to mutate
}
```

---

## 9. JSON export and import

Add export/import methods to your domain repository:

```rust
// In ReadingRepository:

pub fn export_json(&self) -> String {
    let books: Vec<serde_json::Value> = self
        .repo.query()
        .where_eq("user_id", self.user_id)
        .where_eq("deleted", false)
        .fetch()
        .unwrap_or_default()
        .iter()
        .map(|b| serde_json::json!({
            "title":  b.title,
            "author": b.author,
            "pages":  b.pages,
            "read":   b.read,
        }))
        .collect();

    serde_json::to_string_pretty(&serde_json::json!({ "books": books }))
        .unwrap_or_default()
}

pub fn import_json(&self, json: &str) -> Result<usize, DbError> {
    let root: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| DbError::Validation(e.to_string()))?;

    let items = root["books"].as_array()
        .ok_or_else(|| DbError::Validation("expected {\"books\":[…]}".into()))?;

    let mut count = 0;
    for item in items {
        let title  = item["title"].as_str().unwrap_or("").trim();
        if title.is_empty() { continue; }
        let author = item["author"].as_str().unwrap_or("Unknown");
        let pages  = item["pages"].as_i64().unwrap_or(0);
        let read   = item["read"].as_bool().unwrap_or(false);
        let mut book = Book::new(self.user_id, title, author, pages);
        book.read = read;
        self.repo.insert(book)?;
        count += 1;
    }
    Ok(count)
}
```

---

## 10. Add indexes for performance

Add `indexes()` to your `DbRecord` impl. The lib creates them automatically
the first time `db.repository::<T>()` is called.

```rust
// In impl DbRecord for Book:

fn indexes() -> Vec<IndexDef> {
    vec![
        // Covers: WHERE user_id = ? AND deleted = 0
        IndexDef::new(&["user_id", "deleted"]),

        // Covers: WHERE user_id = ? AND read = 0 ORDER BY updated_at
        IndexDef::new(&["user_id", "read", "updated_at"]),
    ]
}
```

### When to add an index

Add an index for **any column combination you filter on frequently**:

```rust
// You always do this?
.where_eq("user_id", …).where_eq("status", …)

// Add:
IndexDef::new(&["user_id", "status"])
```

No index is needed for `id` — it is the primary key and already indexed.

---

## 11. Admin — queries without user scope

`AppState` exposes `raw_*_repo()` methods that return the unscoped
`Repository<T>`. Admin code uses these to query across all users.

```rust
// src/app/admin_screen.rs

pub fn show_admin_stats(state: &AppState, ui: &mut egui::Ui) {
    // Total tasks across ALL users (no user_id filter)
    let total = state.raw_task_repo()
        .query()
        .where_eq("deleted", false)
        .count()
        .unwrap_or(0);

    ui.label(format!("Total tasks in DB: {total}"));

    // All books ever added by any user
    let all_books: Vec<Book> = state.raw_book_repo()
        .query()
        .where_eq("deleted", false)
        .order_by("user_id", Dir::Asc)
        .fetch()
        .unwrap_or_default();

    for book in &all_books {
        ui.label(format!("[user {}] {}", book.user_id, book.title));
    }
}
```

Expose the raw repo from `AppState`:

```rust
// src/core/state.rs
pub fn raw_book_repo(&self) -> &Repository<Book> { &self.book_repo }
```

---

## 12. Add a brand-new domain (full walkthrough)

This is the complete checklist for adding "Notes" to the application.

### Step 1 — Create the struct file

```
akTool/src/notes/note.rs
```

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
        Self { id: 0, user_id, content: content.into(), pinned: false, deleted: false, updated_at: now() }
    }
}

impl DbRecord for Note {
    fn table_name() -> &'static str { "notes" }

    fn columns() -> Vec<Column> {
        vec![
            Column::new("user_id",    ColType::Integer).not_null(),
            Column::new("content",    ColType::Text).not_null(),
            Column::new("pinned",     ColType::Integer).not_null().default("0"),
            Column::new("deleted",    ColType::Integer).not_null().default("0"),
            Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
        ]
    }

    fn indexes() -> Vec<IndexDef> {
        vec![ IndexDef::new(&["user_id", "deleted", "pinned"]) ]
    }

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

    fn to_params(&self) -> Vec<(&'static str, SqlValue)> {
        vec![
            ("user_id",    self.user_id.into()),
            ("content",    self.content.clone().into()),
            ("pinned",     self.pinned.into()),
            ("deleted",    self.deleted.into()),
            ("updated_at", now().into()),
        ]
    }

    fn id(&self) -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
    fn set_id(&mut self, id: i64) { self.id = id; }
}
```

### Step 2 — Create the domain repository

```
akTool/src/notes/repository.rs
```

```rust
use db_lib::{DbError, Dir, Repository};
use super::note::Note;

pub struct NotesRepository {
    repo:    Repository<Note>,
    user_id: i64,
}

impl NotesRepository {
    pub fn new(repo: Repository<Note>, user_id: i64) -> Self {
        Self { repo, user_id }
    }

    pub fn add(&self, content: &str) -> Result<i64, DbError> {
        if content.trim().is_empty() {
            return Err(DbError::Validation("note cannot be empty".into()));
        }
        self.repo.insert(Note::new(self.user_id, content.trim()))
    }

    pub fn pin(&self, id: i64) -> Result<bool, DbError> {
        self.repo.update(id, |n| { n.pinned = true; })
    }

    pub fn unpin(&self, id: i64) -> Result<bool, DbError> {
        self.repo.update(id, |n| { n.pinned = false; })
    }

    pub fn edit(&self, id: i64, new_content: &str) -> Result<bool, DbError> {
        let c = new_content.to_string();
        self.repo.update(id, move |n| { n.content = c; })
    }

    pub fn remove(&self, id: i64) -> Result<bool, DbError> {
        self.repo.delete(id)
    }

    pub fn all(&self) -> Vec<Note> {
        self.repo.query()
            .where_eq("user_id", self.user_id)
            .where_eq("deleted", false)
            .order_by("pinned",     Dir::Desc)  // pinned first
            .order_by("updated_at", Dir::Desc)  // then newest
            .fetch()
            .unwrap_or_default()
    }

    pub fn pinned(&self) -> Vec<Note> {
        self.repo.query()
            .where_eq("user_id", self.user_id)
            .where_eq("pinned",  true)
            .where_eq("deleted", false)
            .fetch()
            .unwrap_or_default()
    }

    pub fn search(&self, query: &str) -> Vec<Note> {
        self.repo.query()
            .where_eq("user_id",     self.user_id)
            .where_eq("deleted",     false)
            .where_like("content",   format!("%{}%", query))
            .fetch()
            .unwrap_or_default()
    }
}
```

### Step 3 — Create `mod.rs`

```
akTool/src/notes/mod.rs
```

```rust
pub mod note;
pub mod repository;
```

### Step 4 — Register in `lib.rs`

```rust
// src/lib.rs
pub mod notes;    // ← add this line
```

### Step 5 — Add to `AppState`

```rust
// src/core/state.rs
use crate::notes::note::Note;
use crate::notes::repository::NotesRepository;

pub struct AppState {
    // … existing fields …
    note_repo: Repository<Note>,   // ← add
}

impl AppState {
    pub fn new(db: Db) -> Result<Self, DbError> {
        Ok(Self {
            // … existing repos …
            note_repo: db.repository::<Note>()?,   // ← add
            // The CREATE TABLE IF NOT EXISTS runs here automatically.
            db, user_id: 1,
        })
    }

    // … existing accessors …

    pub fn notes(&self) -> NotesRepository {               // ← add
        NotesRepository::new(self.note_repo.clone(), self.user_id)
    }
}
```

### Step 6 — Use in the UI

```rust
// In App::update() or a dedicated ui_notes() method:

let all_notes = self.state.notes().all();
for note in &all_notes {
    ui.label(&note.content);
}
if ui.button("Add note").clicked() {
    self.state.notes().add("New note").ok();
}
```

**That is all.** The `notes` table is created automatically on first launch.
No migration scripts, no manual SQL, no rusqlite anywhere in the notes feature.

---

## 13. Cheat sheet

### Full import for a model file

```rust
use db_lib::{ColType, Column, DbError, DbRecord, IndexDef, SqlValue, ValueSet, now};
```

### Full import for a repository file

```rust
use db_lib::{DbError, Dir, Repository};
```

### `impl DbRecord` skeleton

```rust
impl DbRecord for MyStruct {
    fn table_name() -> &'static str { "my_table" }

    fn columns() -> Vec<Column> { vec![
        Column::new("user_id",    ColType::Integer).not_null(),
        Column::new("my_field",   ColType::Text).not_null(),
        Column::new("deleted",    ColType::Integer).not_null().default("0"),
        Column::new("updated_at", ColType::Integer).not_null().default("(unixepoch())"),
    ]}

    fn from_values(v: &ValueSet) -> Result<Self, DbError> {
        Ok(MyStruct {
            id:         v.get("id")?.as_i64()?,
            user_id:    v.get("user_id")?.as_i64()?,
            my_field:   v.get("my_field")?.as_text()?,
            deleted:    v.get("deleted")?.as_bool()?,
            updated_at: v.get("updated_at")?.as_i64()?,
        })
    }

    fn to_params(&self) -> Vec<(&'static str, SqlValue)> { vec![
        ("user_id",    self.user_id.into()),
        ("my_field",   self.my_field.clone().into()),
        ("deleted",    self.deleted.into()),
        ("updated_at", now().into()),
    ]}

    fn id(&self)               -> Option<i64> { if self.id > 0 { Some(self.id) } else { None } }
    fn set_id(&mut self, id: i64)              { self.id = id; }
}
```

### Common query patterns

```rust
// All non-deleted for current user, newest first
repo.query().where_eq("user_id", uid).where_eq("deleted", false).order_by("updated_at", Dir::Desc).fetch()?

// Count
repo.query().where_eq("user_id", uid).where_eq("done", false).count()?

// Search
repo.query().where_like("title", format!("%{}%", q)).fetch()?

// Paginate (page 0 = first page)
repo.query().order_by("id", Dir::Asc).limit(page_size).offset(page * page_size).fetch()?

// Admin: all users, all rows
repo.query().where_eq("deleted", false).fetch()?
```

### Checklist for a new domain

```
□  src/<domain>/model.rs     — struct + impl DbRecord
□  src/<domain>/repository.rs— domain methods
□  src/<domain>/mod.rs       — pub mod model; pub mod repository;
□  src/lib.rs                — pub mod <domain>;
□  src/core/state.rs         — add Repository<Model> field
                             — add db.repository::<Model>()? in new()
                             — add pub fn <domain>() accessor
□  src/app/app.rs            — call self.state.<domain>().method() in UI
```
