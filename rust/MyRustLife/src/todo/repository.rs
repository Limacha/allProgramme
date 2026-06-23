use crate::{core::state::State, todo::Task};

use akgine::database::{DataBase, DbError, Repository};

use std::sync::{Arc, Mutex};

pub fn getUserTasks(state: Arc<Mutex<State>>) -> Result<Vec<Task>, DbError> {
    let db: DataBase = state.lock().unwrap().db.clone();
    let repo: Repository<Task> = db.getRepository::<Task>();
    let userId: i64 = state.lock().unwrap().userId.clone();
    repo.query().where_eq("user_id", userId).fetch()
}

pub fn newUserTask(state: Arc<Mutex<State>>, task: Task) -> Result<i64, DbError> {
    let db: DataBase = state.lock().unwrap().db.clone();
    let repo: Repository<Task> = db.getRepository::<Task>();
    // let userId: i64 = state.lock().unwrap().userId.clone();
    repo.insert(task)
}

pub fn test(state: Arc<Mutex<State>>) -> Result<i64, DbError> {
    let userId: i64 = state.lock().unwrap().userId.clone();
    newUserTask(state, Task::new(userId, "working".to_string()))
}
