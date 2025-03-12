use sqlx::PgPool;
use crate::project::{Project, ProjectStatus}; // Assuming your `Project` and `ProjectStatus` structs are defined
use std::error::Error;

pub struct ProjectService {
    pool: PgPool,
}

impl ProjectService {
    pub fn new(pool: PgPool) -> Self {
        ProjectService { pool }
    }

    pub async fn create_project(&self, name: &str, budget: Option<f64>, client_id: Option<i32>) -> Result<Project, Box<dyn Error>> {
        Project::create(&self.pool, name, budget, client_id).await
    }

    pub async fn update_project(&self, project: &Project) -> Result<(), Box<dyn Error>> {
        project.update(&self.pool).await
    }

    pub async fn transition_project_status(&self, project_id: i32, new_status: ProjectStatus) -> Result<(), Box<dyn Error>> {
        let mut project = match Project::get_by_id(project_id, &self.pool).await? {
            Some(project) => project,
            None => return Err(Box::new(sqlx::Error::RowNotFound)),
        };

        project.transition_status(new_status, &self.pool).await
    }

    pub async fn get_project(&self, project_id: i32) -> Result<Option<Project>, Box<dyn Error>> {
        Project::get_by_id(project_id, &self.pool).await
    }
}
