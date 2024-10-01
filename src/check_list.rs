use crate::task::Task;

pub struct CheckList {
    title: String,
    tasks: Vec<Task>,
}

impl CheckList {
    pub fn new(title: &str) -> Self {
        return Self {
            title: title.to_string(),
            tasks: Vec::new(),
        };
    }

    pub fn load(title: &str, tasks: Vec<Task>) -> Self {
        return Self {
            title: title.to_string(),
            tasks,
        };
    }

    pub fn get_title(&self) -> &str {
        return self.title.as_str();
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        return &self.tasks;
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
