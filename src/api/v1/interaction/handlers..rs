use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::interaction::NewInteraction;

pub async fn log_interaction_handler(
    pool: web::Data<PgPool>,
    interaction: web::Json<NewInteraction>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query(
        r#"
        INSERT INTO interactions (freelancer_id, project_id, interaction_type, interaction_date)
        VALUES ($1, $2, $3, NOW())
        "#,
    )
    .bind(&interaction.freelancer_id)
    .bind(&interaction.project_id)
    .bind(&interaction.interaction_type)
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().body("Interaction logged successfully"))
}