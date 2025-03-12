use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::error::ApiError;

pub async fn create_milestone_handler(
    pool: web::Data<PgPool>,
    web::Path(contract_id): web::Path<u32>,
    milestone: web::Json<NewContractMilestone>,
) -> Result<HttpResponse, ApiError> {
    let new_milestone = milestone.into_inner();

    let created_milestone = sqlx::query_as::<_, ContractMilestone>(
        r#"
        INSERT INTO contract_milestones (contract_id, title, description, due_date, status, payment_amount)
        VALUES ($1, $2, $3, $4, 'in_progress', $5)
        RETURNING *
        "#,
    )
    .bind(contract_id)
    .bind(&new_milestone.title)
    .bind(&new_milestone.description)
    .bind(new_milestone.due_date)
    .bind(new_milestone.payment_amount)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Milestone with title '{}' created for contract ID {}",
        created_milestone.title, contract_id
    );
    Ok(HttpResponse::Created().json(created_milestone))
}

pub async fn get_milestones_handler(
    pool: web::Data<PgPool>,
    web::Path(contract_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let milestones = sqlx::query_as::<_, ContractMilestone>(
        "SELECT * FROM contract_milestones WHERE contract_id = $1",
    )
    .bind(contract_id)
    .fetch_all(pool.as_ref())
    .await?;

    info!("Retrieved milestones for contract ID {}", contract_id);
    Ok(HttpResponse::Ok().json(milestones))
}

pub async fn update_milestone_handler(
    pool: web::Data<PgPool>,
    web::Path((contract_id, milestone_id)): web::Path<(u32, u32)>,
    milestone: web::Json<MilestoneUpdate>,
) -> Result<HttpResponse, ApiError> {
    let updated_milestone = milestone.into_inner();

    let milestone = sqlx::query_as::<_, ContractMilestone>(
        r#"
        UPDATE contract_milestones
        SET title = COALESCE($3, title),
            description = COALESCE($4, description),
            due_date = COALESCE($5, due_date),
            payment_amount = COALESCE($6, payment_amount)
        WHERE id = $1 AND contract_id = $2
        RETURNING *
        "#,
    )
    .bind(milestone_id)
    .bind(contract_id)
    .bind(updated_milestone.title.as_deref())
    .bind(updated_milestone.description.as_deref())
    .bind(updated_milestone.due_date)
    .bind(updated_milestone.payment_amount)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Milestone with ID {} for contract ID {} updated successfully",
        milestone_id, contract_id
    );
    Ok(HttpResponse::Ok().json(milestone))
}

pub async fn mark_milestone_complete_handler(
    pool: web::Data<PgPool>,
    web::Path((contract_id, milestone_id)): web::Path<(u32, u32)>,
) -> Result<HttpResponse, ApiError> {
    // Execute the SQL query to update the milestone status
    let rows_affected = sqlx::query(
        r#"
        UPDATE contract_milestones
        SET status = 'completed', completed_at = NOW()
        WHERE id = $1 AND contract_id = $2 AND status = 'in_progress'
        "#,
    )
    .bind(milestone_id) // Bind the milestone ID
    .bind(contract_id)  // Bind the contract ID
    .execute(pool.as_ref()) // Use the database pool
    .await?
    .rows_affected(); // Get the number of rows affected

    // Check if any rows were updated
    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Milestone with ID {} for contract ID {} not found or already completed",
            milestone_id, contract_id
        )));
    }

    // Log the successful update
    info!(
        "Milestone with ID {} for contract ID {} marked as complete",
        milestone_id, contract_id
    );

    // Return a success response
    Ok(HttpResponse::Ok().body("Milestone marked as complete"))
}