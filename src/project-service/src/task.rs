#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub assigned_to: Option<u32>, // Reference to user ID
    pub due_date: Option<DateTime<Utc>>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub comments: Vec<Comment>,
    pub files: Vec<File>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub milestone_id: Option<u32>, // Link to milestone
    pub project_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub struct TaskDependency {
    pub dependent_task_id: u32,
    pub prerequisite_task_id: u32,
}

impl Task {
    pub fn update_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }

    pub fn assign_to(&mut self, user_id: u32) {
        self.assigned_to = Some(user_id);
    }
}


pub async fn add_subtask(
    &mut self, 
    parent_task_id: i32, 
    subtask: Task, 
    pool: &PgPool
) -> Result<(), Box<dyn std::error::Error>> {
    let parent_task = self.tasks.iter_mut().find(|task| task.id == parent_task_id);
    if let Some(task) = parent_task {
        task.subtasks.push(subtask);
        self.update(pool).await?;
    }
    Ok(())
}

pub async fn prioritize_task(
    &mut self, 
    task_id: i32, 
    priority: i32, 
    pool: &PgPool
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id) {
        task.priority = priority;
        self.update(pool).await?;
    }
    Ok(())
}


pub fn link_task_to_milestone(task_id: u32, milestone_id: u32, tasks: &mut Vec<Task>) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.milestone_id = Some(milestone_id);
    }
}

pub fn update_tasks_based_on_milestone(milestone_id: u32, tasks: &mut Vec<Task>) {
    for task in tasks.iter_mut() {
        if task.milestone_id == Some(milestone_id) {
            task.update_status(TaskStatus::Completed);
        }
    }
}


