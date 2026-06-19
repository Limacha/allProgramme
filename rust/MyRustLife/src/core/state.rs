use crate::todo::Task;
use akgine::database::DataBase;
use std::{
    ptr::null,
    sync::{Arc, Mutex},
};

use crate::{
    core::Router,
    database::{dbPath, openDataBase},
};

/// All application-wide state that pages and activities need to read or mutate.
/// Keep this flat; avoid storing large resources here.
pub struct State {
    /// Router for navigation
    pub router: Router,
    /// Lines shown in the debug panel
    pub debug_texts: Vec<String>,
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
        db.register::<Task>();
        let tasks = db.of::<Task>();

        let mut newTask = Task::new(73, "finir cette merde".to_string());
        let id = tasks.insert(newTask);

        let maybe_task = tasks.find(1);

        Self {
            router: Router::new(path),
            debug_texts: vec![dbPath().display().to_string(), format!("{:?}", maybe_task)],
            db,
        }
    }
}

/// Convenience alias — clone the Arc cheaply, not the Mutex contents.
pub type SharedStateRef = Arc<Mutex<State>>;
