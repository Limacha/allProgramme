use crate::todo::Task;
use akgine::database::DataBase;
use std::{
    mem::take,
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

        // registre and get the table/repo
        db.register::<Task>();
        let tasks: akgine::database::Repository<Task> = db.getRepository::<Task>();
        // can be remplace by
        // db.ensureRepository::<Task>();
        let mut new_task = Task::new(73, "finir cette merde".to_string());

        let id: Result<i64, akgine::database::DbError> = tasks.insert(new_task);
        let maybe_task: Result<Option<Task>, akgine::database::DbError> = tasks.find(1);
        // let maybe_task: Result<Option<Task>, akgine::database::DbError> =
        //     tasks.query().where_eq("id", 1).fetch_one();

        Self {
            router: Router::new(path),
            debug_texts: vec![dbPath().display().to_string(), format!("{:?}", maybe_task)],
            db,
        }
    }
}

/// Convenience alias — clone the Arc cheaply, not the Mutex contents.
pub type SharedStateRef = Arc<Mutex<State>>;
