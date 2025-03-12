use actix_web::{web, HttpResponse, Responder};
use crate::database::DbPool;
use crate::models::contract::{Contract, NewContract};

pub async fn create_contract_handler(
    pool: web::Data<DbPool>,
    data: web::Json<NewContract>,
) -> impl Responder {
    match crate::database::queries::create_contract(pool.as_ref(), data.into_inner()).await {
        Ok(contract) => HttpResponse::Created().json(contract),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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