use actix_web::{web, HttpResponse, Responder};
use crate::database::DbPool;
use crate::models::escrow::{EscrowAccount, NewEscrowAccount};
use sqlx::PgPool;

pub async fn create_escrow_account(
    pool: web::Data<PgPool>,
    web::Path(contract_id): web::Path<u32>,
    account: web::Json<EscrowAccount>,
) -> Result<HttpResponse, ApiError> {
    let new_account = account.into_inner();

    let created_account = sqlx::query_as::<_, EscrowAccount>(
        r#"
        INSERT INTO escrow_accounts (contract_id, sender_id, receiver_id, amount, currency, status, release_conditions)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(contract_id)
    .bind(new_account.sender_id)
    .bind(new_account.receiver_id)
    .bind(new_account.amount)
    .bind(new_account.currency)
    .bind(new_account.status)
    .bind(new_account.release_conditions)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Escrow account created successfully for contract ID {}: {:?}",
        contract_id, created_account
    );
    Ok(HttpResponse::Created().json(created_account))
}

pub async fn release_escrow_handler(
    pool: web::Data<PgPool>,
    web::Path(escrow_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE escrow_accounts
        SET status = 'released', released_at = NOW()
        WHERE id = $1 AND status = 'pending'
        "#,
    )
    .bind(escrow_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound("Escrow account not found or already released".to_string()));
    }

    info!("Escrow account with ID {} released successfully", escrow_id);
    Ok(HttpResponse::Ok().body("Escrow account released successfully"))
}

pub async fn create_escrow_handler(
    pool: web::Data<PgPool>,
    web::Path(contract_id): web::Path<u32>,
    amount: web::Json<f32>,
) -> Result<HttpResponse, ApiError> {
    let escrow_amount = amount.into_inner();

    let created_account = sqlx::query_as::<_, EscrowAccount>(
        r#"
        INSERT INTO escrow_accounts (contract_id, sender_id, receiver_id, amount, currency, status, release_conditions)
        VALUES ($1, NULL, NULL, $2, 'USD', 'pending', NULL)
        RETURNING *
        "#,
    )
    .bind(contract_id)
    .bind(escrow_amount)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Escrow account created successfully for contract ID {}: {:?}",
        contract_id, created_account
    );
    Ok(HttpResponse::Created().json(created_account))
}

pub async fn refund_escrow_handler(
    pool: web::Data<PgPool>,
    web::Path(escrow_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE escrow_accounts
        SET status = 'refunded', refunded_at = NOW()
        WHERE id = $1 AND status = 'pending'
        "#,
    )
    .bind(escrow_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound("Escrow account not found or already refunded".to_string()));
    }

    info!("Escrow account with ID {} refunded successfully", escrow_id);
    Ok(HttpResponse::Ok().body("Escrow account refunded successfully"))
}

pub async fn create_contract_with_escrow_handler(
    pool: web::Data<PgPool>,
    contract: web::Json<Contract>,
    amount: web::Json<f32>,
) -> Result<HttpResponse, ApiError> {
    let new_contract = contract.into_inner();
    let escrow_amount = amount.into_inner();

    // Create the contract
    let created_contract = sqlx::query_as::<_, Contract>(
        r#"
        INSERT INTO contracts (client_id, freelancer_id, title, description, value, start_date, end_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(new_contract.client_id)
    .bind(new_contract.freelancer_id)
    .bind(&new_contract.title)
    .bind(&new_contract.description)
    .bind(new_contract.value)
    .bind(new_contract.start_date)
    .bind(new_contract.end_date)
    .fetch_one(pool.as_ref())
    .await?;

    // Create the escrow account
    let escrow_account = sqlx::query_as::<_, EscrowAccount>(
        r#"
        INSERT INTO escrow_accounts (contract_id, sender_id, receiver_id, amount, currency, status, release_conditions)
        VALUES ($1, $2, $3, $4, 'USD', 'pending', NULL)
        RETURNING *
        "#,
    )
    .bind(created_contract.id)
    .bind(new_contract.client_id)
    .bind(new_contract.freelancer_id)
    .bind(escrow_amount as f64)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Contract with ID {} and escrow account created successfully",
        created_contract.id
    );
    Ok(HttpResponse::Created().json((created_contract, escrow_account)))
}
