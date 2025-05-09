use sqlx::{PgPool, query, query_as, Error};
use chrono::{Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use crate::models::{ProjectTemplate, Project};
use tokio::task;

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub budget: Option<f64>,
    pub client_id: Option<i32>,
    pub freelancer_id: Option<i32>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub dependencies: Vec<u32>,
    pub members: Vec<ProjectMember>,
    pub tasks: Vec<Task>,
    pub workflow_id: Option<i32>,
    pub dependent_projects: Vec<i32>,
    pub files: Option<Vec<String>>,
    pub client_name: Option<String>,
    pub priority: Option<i32>,
    pub communication_channels: Option<Vec<String>>,
    pub enable_milestones: bool,
    pub milestones: Vec<Milestones>,
    pub contract_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectOverview {
    pub id: i32,
    pub name: String,
    pub status: ProjectStatus,
    pub client_username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ProjectFreelancer {
    pub project_id: i32,
    pub freelancer_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDetails {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: String,
    pub client_username: String,
    pub freelancers: Vec<String>,
    pub deadlines: Vec<Deadline>,
    pub progress: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStage {
    pub id: i32,
    pub workflow_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub order: i32,
    pub status: WorkflowStageStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowStageStatus {
    NotStarted,
    InProgress,
    Completed,
    Skipped,
}

impl Project {
    pub async fn create(pool: &PgPool, name: &str, budget: Option<f64>, client_id: Option<i32>) -> Result<Project, sqlx::Error> {
        let new_project = Project {
            id: 0,
            name: name.to_string(),
            budget,
            client_id,
            freelancer_id: None,
            start_date: None,
            end_date: None,
            category: None,
            description: None,
            files: None,
            dependencies: Vec::new(),
            client_name: None,
            priority: None,
            communication_channels: None,
            enable_milestones: false,
            contract_id: None,
            dependent_projects: Vec::new(),
            members: Vec::new(),
            tasks: Vec::new(),
            workflow_id: None,
        };

        let result = query!(
            "INSERT INTO projects (name, budget, client_id, freelancer_id, start_date, end_date, category, description, files, enable_milestones, contract_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING id, name, budget, client_id, freelancer_id, start_date, end_date, category, description, files, enable_milestones, contract_id",
            new_project.name,
            new_project.budget,
            new_project.client_id,
            new_project.freelancer_id,
            new_project.start_date,
            new_project.end_date,
            new_project.category,
            new_project.description,
            new_project.files,
            new_project.enable_milestones,
            new_project.contract_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: result.id,
            name: result.name,
            budget: result.budget,
            client_id: result.client_id,
            freelancer_id: result.freelancer_id,
            start_date: result.start_date,
            end_date: result.end_date,
            category: result.category,
            description: result.description,
            files: result.files,
            enable_milestones: result.enable_milestones,
            contract_id: result.contract_id,
            dependent_projects: Vec::new(),
            members: Vec::new(),
            tasks: Vec::new(),
            workflow_id: None,
        })
    }

    pub async fn update(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        query!(
            "UPDATE projects SET 
            name = $1, 
            budget = $2, 
            client_id = $3, 
            freelancer_id = $4, 
            start_date = $5, 
            end_date = $6, 
            category = $7, 
            description = $8, 
            files = $9, 
            enable_milestones = $10, 
            contract_id = $11
            WHERE id = $12",
            self.name,
            self.budget,
            self.client_id,
            self.freelancer_id,
            self.start_date,
            self.end_date,
            self.category,
            self.description,
            self.files,
            self.enable_milestones,
            self.contract_id,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(id: i32, pool: &PgPool) -> Result<Option<Project>, sqlx::Error> {
        let result = query_as!(
            Project,
            "SELECT * FROM projects WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
        let result = query_as!(
            Project,
            "SELECT * FROM projects"
        )
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    pub async fn transition_status(
        &mut self,
        new_status: ProjectStatus,
        pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        let previous_status = self.status.clone();
        self.status = new_status;

        let status_history = ProjectStatusHistory {
            id: None,
            project_id: self.id,
            previous_status,
            new_status,
            created_at: Utc::now(),
        };

        query!(
            "INSERT INTO project_status_history (project_id, previous_status, new_status, created_at)
            VALUES ($1, $2, $3, $4)",
            status_history.project_id,
            status_history.previous_status,
            status_history.new_status,
            status_history.created_at
        )
        .execute(pool)
        .await?;

        self.update(pool).await
    }

    pub async fn get_status_history(
        project_id: i32,
        pool: &PgPool,
    ) -> Result<Vec<ProjectStatusHistory>, sqlx::Error> {
        let result = query_as!(
            ProjectStatusHistory,
            "SELECT * FROM project_status_history WHERE project_id = $1 ORDER BY created_at DESC",
            project_id
        )
        .fetch_all(pool)
        .await?;

        Ok(result)
    }
}

// Enums

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectStatus {
    Initiated,
    Planned,
    InPreparation,
    InProgress,
    OnHold,
    Completed,
    Cancelled,
    Deferred,
}

impl ProjectStatus {
    pub fn to_string(&self) -> String {
        match self {
            ProjectStatus::Initiated => "Initiated".to_string(),
            ProjectStatus::Planned => "Planned".to_string(),
            ProjectStatus::InPreparation => "InPreparation".to_string(),
            ProjectStatus::InProgress => "InProgress".to_string(),
            ProjectStatus::OnHold => "OnHold".to_string(),
            ProjectStatus::Completed => "Completed".to_string(),
            ProjectStatus::Cancelled => "Cancelled".to_string(),
            ProjectStatus::Deferred => "Deferred".to_string(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStatusHistory {
    pub id: Option<i32>,
    pub project_id: i32,
    pub previous_status: ProjectStatus,
    pub new_status: ProjectStatus,
    pub created_at: DateTime<Utc>,
}



#[derive(Debug, Clone)]
pub enum ProjectVisibility {
    Public,
    Private,
    Shared,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub project_id: u32,
    pub depends_on: u32, // ID of the project this project depends on
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTemplate {
    id: i32,
    name: String,
    default_budget: Option<f64>,
    default_category: Option<String>,
    default_description: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFee {
    pub id: u32,
    pub contract_id: u32,
    pub amount: f32,
    pub fee_type: FeeType,
    pub charged_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeType {
    Fixed,  // Fixed fee amount
    Percentage(f32), // Percentage of the contract value
}


pub async fn adjust_dates_based_on_dependencies(
    &mut self, 
    pool: &PgPool
) -> Result<(), Box<dyn std::error::Error>> {
    if !self.dependencies.is_empty() {
        // Find the latest end date of dependent projects
        let result: Option<NaiveDateTime> = sqlx::query!(
            "SELECT MAX(end_date) FROM projects WHERE id = ANY($1)",
            &self.dependencies
        )
        .fetch_one(pool)
        .await?
        .max; // Accessing the value of the MAX query
        
        if let Some(latest_end_date) = result {
            self.start_date = Some(latest_end_date + chrono::Duration::days(1));
            self.update(pool).await?;
        }
    }
    Ok(())
}

pub async fn toggle_milestones(
    &mut self, 
    enable_milestones: bool
) -> Result<(), Box<dyn std::error::Error>> {
    self.enable_milestones = enable_milestones;
    if !enable_milestones {
        self.milestones.clear();  // Remove milestones if the feature is disabled
    }
    Ok(())
}


pub async fn execute_parallel_stages(
    project: &Project, 
    pool: &PgPool
) -> Result<(), Box<dyn std::error::Error>> {
    let workflow = Workflow::belonging_to(project).first(pool).await?;
    let stages = WorkflowStage::belonging_to(&workflow).fetch_all(pool).await?;

    // Group independent stages
    let independent_stages: Vec<_> = stages.into_iter()
        .filter(|stage| stage.is_independent())
        .collect();

    let mut handles = vec![];

    // Spawn tasks for independent stages
    for stage in independent_stages {
        let conn_clone = pool.clone(); // Clone the connection pool for parallel execution
        let handle = task::spawn(async move {
            execute_stage(&stage, project, &conn_clone).await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await??;  // Unwrap the result of the task
    }

    Ok(())
}


pub async fn execute_custom_stage_transitions(
    &mut self, 
    pool: &PgPool
) -> Result<(), Box<dyn std::error::Error>> {
    if self.status == WorkflowStageStatus::Skipped {
        // Skip certain stages based on conditions
        return Ok(());
    }

    // Continue with execution as normal
    execute_stage(self, pool).await
}


pub async fn execute_workflow_optimized(
    project: &Project, 
    pool: &PgPool
) -> Result<(), Box<dyn std::error::Error>> {
    let workflow = Workflow::belonging_to(project).first(pool).await?;
    let stages = WorkflowStage::belonging_to(&workflow).fetch_all(pool).await?;

    let mut status_cache = HashMap::new();
    for stage in stages {
        let status = status_cache.entry(stage.id).or_insert_with(|| {
            check_status_condition(&stage, project)
        });

        if *status && should_execute_stage(&stage, project) {
            execute_stage(&stage, project, pool).await?;
        }
    }

    Ok(())
}
pub async fn can_user_access_project(
    user_id: i32, 
    project_id: i32, 
    required_permission: &str, 
    pool: &PgPool
) -> Result<bool, Error> {
    // Check if the user is a member of the project
    let user_is_member = sqlx::query!(
        r#"
        SELECT COUNT(*) > 0 AS user_is_member
        FROM project_users
        WHERE project_id = $1 AND user_id = $2
        "#,
        project_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    if !user_is_member.user_is_member {
        return Ok(false);
    }

    // Check if the user's role has the required permission
    let has_permission = sqlx::query!(
        r#"
        SELECT COUNT(*) > 0 AS has_permission
        FROM roles
        INNER JOIN project_users ON roles.id = project_users.role_id
        INNER JOIN role_permissions ON roles.id = role_permissions.role_id
        INNER JOIN permissions ON role_permissions.permission_id = permissions.id
        WHERE project_users.project_id = $1
        AND project_users.user_id = $2
        AND permissions.name = $3
        "#,
        project_id,
        user_id,
        required_permission
    )
    .fetch_one(pool)
    .await?;

    Ok(has_permission.has_permission)
}
