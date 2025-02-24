#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending,
        }
    }

    pub fn mark_complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
}

pub struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    pub fn new() -> Self {
        ToDoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
        println!("✅ Task added successfully!");
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks found.");
            return;
        }
        for task in &self.tasks {
            let status = match task.status {
                TaskStatus::Pending => "⏳ Pending",
                TaskStatus::Completed => "✅ Completed",
            };
            println!("[{}] {} - {}", task.id, task.description, status);
        }
    }

    pub fn mark_task_completed(&mut self, task_id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.mark_complete();
            println!("🎉 Task {} marked as completed!", task_id);
        } else {
            println!("❌ Task ID not found.");
        }
    }

    pub fn delete_task(&mut self, task_id: usize) {
        if let Some(index) = self.tasks.iter().position(|t| t.id == task_id) {
            self.tasks.remove(index);
            println!("🗑️ Task {} deleted successfully!", task_id);
        } else {
            println!("❌ Task ID not found.");
        }
    }
}
