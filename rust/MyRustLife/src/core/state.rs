use akgine::database::DataBase;
use akgine::navigation::Router;
use std::sync::{Arc, Mutex};

use crate::database::{dbPath, openDataBase};

/// All application-wide state that pages and activities need to read or mutate.
/// Keep this flat; avoid storing large resources here.
pub struct State {
    /// Router for navigation
    pub router: Router,
    /// Lines shown in the debug panel
    pub debug_texts: Vec<String>,
    pub userId: i64,
    // the database
    pub db: DataBase,
}

// impl State {
//     fn new(router: Router, debug_texts: Vec<String>, db: DataBase) -> Self {
//         Self {
//             router,
//             debug_texts,
//             db,
//         }
//     }
// }

impl Default for State {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        let path: &str = "/DebugActivity";
        #[cfg(not(debug_assertions))]
        let path: &str = "/ReleaseActivity";

        let db: DataBase = openDataBase().expect("Failed to init database");

        Self {
            router: Router::new(path),
            debug_texts: vec![dbPath().display().to_string()],
            userId: 25,
            db,
        }
    }
}

/// Convenience alias — clone the Arc cheaply, not the Mutex contents.
pub type SharedStateRef = Arc<Mutex<State>>;
