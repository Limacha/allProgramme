// src/watchList/repository.rs
//
// WatchListRepository now stores a Db handle (not pre-built Repository<T>).
// Sub-repos are created on demand via db.of::<T>() — zero cost (Arc clone).

use db_lib::{Db, DbError, Dir};

use super::anime::{Anime, AnimeStatus};
use super::scan::{Scan, ScanStatus};

pub struct WatchListRepository {
    db:      Db,
    user_id: i64,
}

impl WatchListRepository {
    pub fn new(db: Db, user_id: i64) -> Self {
        Self { db, user_id }
    }

    fn anime(&self) -> db_lib::Repository<Anime> { self.db.of::<Anime>() }
    fn scans(&self) -> db_lib::Repository<Scan>  { self.db.of::<Scan>() }

    // ══════════════════════════════════════════════════════════════════════════
    // ANIME
    // ══════════════════════════════════════════════════════════════════════════

    pub fn add_anime(&self, title: &str) -> Result<i64, DbError> {
        let t = title.trim();
        if t.is_empty() { return Err(DbError::Validation("title cannot be empty".into())); }
        self.anime().insert(Anime::new(self.user_id, t))
    }

    pub fn set_anime_status(&self, id: i64, status: AnimeStatus) -> Result<bool, DbError> {
        self.anime().update(id, move |a| { a.status = status; })
    }

    pub fn next_episode(&self, id: i64) -> Result<bool, DbError> {
        self.anime().update(id, |a| { a.episode += 1; })
    }

    pub fn set_episode(&self, id: i64, ep: i64) -> Result<bool, DbError> {
        self.anime().update(id, move |a| { a.episode = ep; })
    }

    pub fn set_anime_notes(&self, id: i64, notes: &str) -> Result<bool, DbError> {
        let n = notes.to_string();
        self.anime().update(id, move |a| { a.notes = n; })
    }

    pub fn remove_anime(&self, id: i64) -> Result<bool, DbError> {
        self.anime().delete(id)
    }

    pub fn all_anime(&self) -> Vec<Anime> {
        self.anime().query()
            .where_eq("user_id", self.user_id)
            .where_eq("deleted", false)
            .order_by("id", Dir::Asc)
            .fetch().unwrap_or_default()
    }

    pub fn anime_by_status(&self, status: &AnimeStatus) -> Vec<Anime> {
        self.anime().query()
            .where_eq("user_id", self.user_id)
            .where_eq("status",  status.as_str())
            .where_eq("deleted", false)
            .order_by("updated_at", Dir::Desc)
            .fetch().unwrap_or_default()
    }

    pub fn currently_watching(&self) -> Vec<Anime> {
        self.anime_by_status(&AnimeStatus::Watching)
    }

    pub fn anime_count_by_status(&self) -> std::collections::HashMap<String, i64> {
        let mut map = std::collections::HashMap::new();
        for s in AnimeStatus::ALL {
            let n = self.anime().query()
                .where_eq("user_id", self.user_id)
                .where_eq("status",  s.as_str())
                .where_eq("deleted", false)
                .count().unwrap_or(0);
            map.insert(s.label().to_string(), n);
        }
        map
    }

    // ══════════════════════════════════════════════════════════════════════════
    // SCAN
    // ══════════════════════════════════════════════════════════════════════════

    pub fn add_scan(&self, title: &str) -> Result<i64, DbError> {
        let t = title.trim();
        if t.is_empty() { return Err(DbError::Validation("title cannot be empty".into())); }
        self.scans().insert(Scan::new(self.user_id, t))
    }

    pub fn set_chapter(&self, id: i64, ch: f64) -> Result<bool, DbError> {
        self.scans().update(id, move |s| { s.chapter = ch; })
    }

    pub fn advance_chapter(&self, id: i64, by: f64) -> Result<bool, DbError> {
        self.scans().update(id, move |s| { s.chapter += by; })
    }

    pub fn set_scan_status(&self, id: i64, status: ScanStatus) -> Result<bool, DbError> {
        self.scans().update(id, move |s| { s.status = status; })
    }

    pub fn set_scan_notes(&self, id: i64, notes: &str) -> Result<bool, DbError> {
        let n = notes.to_string();
        self.scans().update(id, move |s| { s.notes = n; })
    }

    pub fn remove_scan(&self, id: i64) -> Result<bool, DbError> {
        self.scans().delete(id)
    }

    pub fn all_scans(&self) -> Vec<Scan> {
        self.scans().query()
            .where_eq("user_id", self.user_id)
            .where_eq("deleted", false)
            .order_by("id", Dir::Asc)
            .fetch().unwrap_or_default()
    }

    pub fn currently_reading(&self) -> Vec<Scan> {
        self.scans().query()
            .where_eq("user_id", self.user_id)
            .where_eq("status",  ScanStatus::Reading.as_str())
            .where_eq("deleted", false)
            .order_by("updated_at", Dir::Desc)
            .fetch().unwrap_or_default()
    }

    // ── JSON export ───────────────────────────────────────────────────────────

    pub fn export_json(&self) -> String {
        let anime = self.all_anime();
        let scans = self.all_scans();
        serde_json::to_string_pretty(&serde_json::json!({
            "anime": anime.iter().map(|a| serde_json::json!({
                "id": a.id, "title": a.title,
                "status": a.status.as_str(), "episode": a.episode,
            })).collect::<Vec<_>>(),
            "scans": scans.iter().map(|s| serde_json::json!({
                "id": s.id, "title": s.title,
                "status": s.status.as_str(), "chapter": s.chapter,
            })).collect::<Vec<_>>(),
        })).unwrap_or_default()
    }
}
