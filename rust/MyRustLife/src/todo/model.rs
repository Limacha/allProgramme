use crate::core::state::AppState;
use crate::todo::task::Task;
use akgine::database::Repository;

pub fn add_new_task(state: &AppState, title: &str) {
    // 1. Create a Repository specifically for the Task model using the centralized Db
    let mut task_repo = Repository::<Task>::new(&state.db);

    // 2. Create your struct
    let new_task = Task {
        id: 0, // id is auto-managed by the library
        title: title.to_string(),
        done: false,
    };

    // 3. Insert it! (Assuming Repository has an insert method)
    task_repo.insert(&new_task).expect("Failed to insert task");
}
