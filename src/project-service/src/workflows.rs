use sqlx::{PgPool, Error};
use std::error::Error as StdError;
use crate::models::Workflow;

pub async fn create_workflow(pool: &PgPool, workflow: &Workflow) -> Result<Workflow, Box<dyn StdError>> {
    let result = sqlx::query_as!(
        Workflow,
        r#"
        INSERT INTO workflows (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description
        "#,
        workflow.name,
        workflow.description
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(result)
}

pub async fn read_workflow(pool: &PgPool, workflow_id: i32) -> Result<Workflow, Box<dyn StdError>> {
    let result = sqlx::query_as!(
        Workflow,
        r#"
        SELECT id, name, description
        FROM workflows
        WHERE id = $1
        "#,
        workflow_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(result)
}

pub async fn update_workflow(pool: &PgPool, workflow_id: i32, updated_workflow: &Workflow) -> Result<Workflow, Box<dyn StdError>> {
    let result = sqlx::query_as!(
        Workflow,
        r#"
        UPDATE workflows
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description
        "#,
        updated_workflow.name,
        updated_workflow.description,
        workflow_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(result)
}

pub async fn delete_workflow(pool: &PgPool, workflow_id: i32) -> Result<(), Box<dyn StdError>> {
    sqlx::query!(
        r#"
        DELETE FROM workflows
        WHERE id = $1
        "#,
        workflow_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(())
}
