#![allow(non_snake_case)]
pub mod database;
pub mod navigation;
pub mod widgets;

// ── Utility ───────────────────────────────────────────────────────────────────

/// Returns the current Unix timestamp in seconds.
///
/// Exported so application code can use it in `DbRecord::to_params()`
/// without depending on std::time directly.
pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
