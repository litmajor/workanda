use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub assigned_to: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub assigned_to: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: TaskPriority,
}

impl TaskPriority {
    pub fn to_string(&self) -> String {
        match self {
            TaskPriority::Low => "Low".to_string(),
            TaskPriority::Medium => "Medium".to_string(),
            TaskPriority::High => "High".to_string(),
        }
    }
}

impl TaskStatus {
    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::ToDo => "ToDo".to_string(),
            TaskStatus::InProgress => "InProgress".to_string(),
            TaskStatus::Completed => "Completed".to_string(),
        }
    }
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            assigned_to: None,
            comments: vec![],
            files: vec![],
            completed: false,
        }
    }

    pub fn assign_to(&mut self, user: User) {
        self.assigned_to = Some(user);
    }

    pub fn mark_as_completed(&mut self) {
        self.completed = true;
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn attach_file(&mut self, file: File) {
        self.files.push(file);
    }
}

pub fn can_edit_task(user: &User, task: &Task) -> bool {
    match user.role {
        Role::Freelancer => true,
        Role::Client => false,
    }
}

pub fn can_view_task(user: &User, task: &Task) -> bool {
    match user.role {
        Role::Freelancer | Role::Client => true,
    }
}

pub fn can_comment(user: &User, task: &Task) -> bool {
    match user.role {
        Role::Freelancer | Role::Client => true,
    }
}

pub fn can_upload_file(user: &User) -> bool {
    match user.role {
        Role::Freelancer | Role::Client => true,
    }
}
