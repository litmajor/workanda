use sqlx::PgPool;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_client_account_handler(
    pool: web::Data<PgPool>,
    client_account: web::Json<ClientAccount>,
) -> Result<HttpResponse, ApiError> {
    let new_account = client_account.into_inner();

    let created_client = sqlx::query_as::<_, ClientAccount>(
        r#"
        INSERT INTO client_accounts (user_id, company_name, industry, location, contact_email)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(new_account.user_id)
    .bind(&new_account.company_name)
    .bind(&new_account.industry)
    .bind(&new_account.location)
    .bind(&new_account.contact_email)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Created client account with ID {}", created_client.id);
    Ok(HttpResponse::Created().json(created_client))
}

pub async fn get_client_by_email_handler(
    pool: web::Data<PgPool>,
    web::Path(email): web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let client = sqlx::query_as::<_, ClientAccount>(
        "SELECT * FROM client_accounts WHERE contact_email = $1",
    )
    .bind(&email)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or(ApiError::NotFound(format!("Client with email {} not found", email)))?;

    info!("Retrieved client account with email {}", email);
    Ok(HttpResponse::Ok().json(client))
}

pub async fn update_client_account_handler(
    pool: web::Data<PgPool>,
    client_account: web::Json<ClientAccount>,
) -> Result<HttpResponse, ApiError> {
    let updated_account = client_account.into_inner();

    let client = sqlx::query_as::<_, ClientAccount>(
        r#"
        UPDATE client_accounts
        SET company_name = COALESCE($2, company_name),
            industry = COALESCE($3, industry),
            location = COALESCE($4, location),
            contact_email = COALESCE($5, contact_email)
        WHERE user_id = $1
        RETURNING *
        "#,
    )
    .bind(updated_account.user_id)
    .bind(&updated_account.company_name)
    .bind(&updated_account.industry)
    .bind(&updated_account.location)
    .bind(&updated_account.contact_email)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Updated client account with ID {}", updated_account.user_id);
    Ok(HttpResponse::Ok().json(client))
}


pub async fn delete_client_account_handler(
    pool: web::Data<PgPool>,
    web::Path(client_id): web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        "DELETE FROM client_accounts WHERE user_id = $1",
    )
    .bind(client_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Client account with ID {} not found",
            client_id
        )));
    }

    info!("Deleted client account with ID {}", client_id);
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_client_account_handler(
    pool: web::Data<DbPool>,
    client_id: web::Path<i32>,
) -> impl Responder {
    match crate::database::queries::get_client_account_by_id(pool.as_ref(), *client_id).await {
        Ok(Some(client)) => HttpResponse::Ok().json(client),
        Ok(None) => HttpResponse::NotFound().body("Client not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}