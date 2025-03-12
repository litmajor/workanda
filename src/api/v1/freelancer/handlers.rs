use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::freelancer::FreelancerAccount;

pub async fn create_freelancer_account_handler(
    pool: web::Data<PgPool>,
    new_freelancer: web::Json<NewFreelancerAccount>,
) -> Result<HttpResponse, ApiError> {
    let new_account = new_freelancer.into_inner();

    let created_account = sqlx::query_as::<_, FreelancerAccount>(
        r#"
        INSERT INTO freelancer_accounts (user_id, hourly_rate, project_pricing, specializations, category, availability, location)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(new_account.user_id)
    .bind(new_account.hourly_rate)
    .bind(new_account.project_pricing)
    .bind(&new_account.specializations)
    .bind(&new_account.category)
    .bind(new_account.availability.to_string())
    .bind(new_account.location)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Created freelancer account with ID {}", created_account.id);
    Ok(HttpResponse::Created().json(created_account))
}

pub async fn fetch_contracts_for_client(
    pool: web::Data<PgPool>,
    web::Path(client_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let contracts = sqlx::query_as::<_, Contract>(
        "SELECT * FROM contracts WHERE client_id = $1",
    )
    .bind(client_id)
    .fetch_all(pool.as_ref())
    .await?;

    info!("Fetched contracts for client ID {}", client_id);
    Ok(HttpResponse::Ok().json(contracts))
}

pub async fn get_freelancer_account_handler(
    pool: web::Data<DbPool>,
    freelancer_id: web::Path<i32>,
) -> impl Responder {
    match crate::database::queries::get_freelancer_account_by_id(pool.as_ref(), *freelancer_id).await {
        Ok(Some(freelancer)) => HttpResponse::Ok().json(freelancer),
        Ok(None) => HttpResponse::NotFound().body("Freelancer not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_freelancer_account_handler(
    pool: web::Data<PgPool>,
    web::Path(freelancer_id): web::Path<i32>,
    updated_data: web::Json<UpdatedFreelancerAccount>,
) -> Result<HttpResponse, ApiError> {
    let updated_account = sqlx::query_as::<_, FreelancerAccount>(
        r#"
        UPDATE freelancer_accounts
        SET hourly_rate = COALESCE($2, hourly_rate),
            project_pricing = COALESCE($3, project_pricing),
            specializations = COALESCE($4, specializations),
            category = COALESCE($5, category),
            availability = COALESCE($6, availability),
            location = COALESCE($7, location)
        WHERE user_id = $1
        RETURNING *
        "#,
    )
    .bind(freelancer_id)
    .bind(updated_data.hourly_rate)
    .bind(updated_data.project_pricing)
    .bind(&updated_data.specializations)
    .bind(&updated_data.category)
    .bind(updated_data.availability.map(|a| a.to_string()))
    .bind(updated_data.location)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Updated freelancer account with ID {}", freelancer_id);
    Ok(HttpResponse::Ok().json(updated_account))
}

pub async fn delete_freelancer_account_handler(
    pool: web::Data<PgPool>,
    web::Path(freelancer_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        "DELETE FROM freelancer_accounts WHERE user_id = $1",
    )
    .bind(freelancer_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Freelancer account not found for ID {}",
            freelancer_id
        )));
    }

    info!("Deleted freelancer account with ID {}", freelancer_id);
    Ok(HttpResponse::NoContent().finish())
}