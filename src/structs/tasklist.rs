use serde::{Deserialize, Serialize};

use super::task::Task;

#[derive(Serialize, Deserialize, Default)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}
