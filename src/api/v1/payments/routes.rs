use sqlx::PgPool;
use crate::models::payment::ContractPayment;


pub async fn create_payment(
    pool: web::Data<PgPool>,
    web::Path(contract_id): web::Path<u32>,
    payment: web::Json<ContractPayment>,
) -> Result<HttpResponse, ApiError> {
    let new_payment = payment.into_inner();

    let created_payment = sqlx::query_as::<_, ContractPayment>(
        r#"
        INSERT INTO payments (contract_id, amount, status, description)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(contract_id)
    .bind(new_payment.amount)
    .bind(new_payment.status)
    .bind(new_payment.description)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Payment created successfully for contract ID {}: {:?}",
        contract_id, created_payment
    );
    Ok(HttpResponse::Created().json(created_payment))
}

pub async fn get_payments(
    pool: web::Data<PgPool>,
    web::Path(contract_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let payments = sqlx::query_as::<_, ContractPayment>(
        "SELECT * FROM payments WHERE contract_id = $1",
    )
    .bind(contract_id)
    .fetch_all(pool.as_ref())
    .await?;

    info!(
        "Retrieved payments for contract ID {} successfully",
        contract_id
    );
    Ok(HttpResponse::Ok().json(payments))
}

pub async fn update_payment_status(
    pool: web::Data<PgPool>,
    web::Path(payment_id): web::Path<u32>,
    payment: web::Json<ContractPayment>,
) -> Result<HttpResponse, ApiError> {
    let updated_payment = payment.into_inner();

    let payment = sqlx::query_as::<_, ContractPayment>(
        r#"
        UPDATE payments
        SET status = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(payment_id)
    .bind(updated_payment.status)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Payment status updated successfully for payment ID {}: {:?}",
        payment_id, payment
    );
    Ok(HttpResponse::Ok().json(payment))
}

pub async fn send_payment_reminders_handler(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let unpaid_contracts = sqlx::query_as::<_, Contract>(
        r#"
        SELECT * FROM contracts
        WHERE status = 'Pending' AND current_date > deadline
        "#,
    )
    .fetch_all(pool.as_ref())
    .await?;

    // Logic to send reminders (e.g., email notifications)
    for contract in unpaid_contracts {
        info!(
            "Sending payment reminder for contract ID {} (Client: {}, Freelancer: {})",
            contract.id, contract.client_id, contract.freelancer_id
        );
        // Example: Call a notification service here
    }

    Ok(HttpResponse::Ok().body("Payment reminders sent successfully"))
}
