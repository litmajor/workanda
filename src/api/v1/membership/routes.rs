use sqlx::PgPool;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use log::{info, error};

#[derive(Debug, Deserialize)]
pub struct UpdateMembershipRequest {
    pub user_id: i32,
    pub new_tier: MembershipTier,
}

#[derive(Debug, Serialize)]
pub struct UpdateMembershipResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum MembershipTier {
    Basic,
    Normal,
    Premium,
    Enterprise,
}

impl MembershipTier {
    pub fn to_string(&self) -> String {
        match self {
            MembershipTier::Basic => "Basic".to_string(),
            MembershipTier::Normal => "Normal".to_string(),
            MembershipTier::Premium => "Premium".to_string(),
            MembershipTier::Enterprise => "Enterprise".to_string(),
        }
    }
}

pub async fn update_membership_handler(
    pool: web::Data<PgPool>,
    update_req: web::Json<UpdateMembershipRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = update_req.user_id;
    let new_tier = update_req.new_tier.to_string();

    // Execute the SQLx query to update the membership tier
    let rows_affected = sqlx::query(
        r#"
        UPDATE users
        SET membership_tier = $1
        WHERE id = $2
        "#,
    )
    .bind(new_tier)
    .bind(user_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!("User with ID {} not found", user_id)));
    }

    info!("User {} upgraded to {:?}", user_id, update_req.new_tier);
    Ok(HttpResponse::Ok().json(UpdateMembershipResponse {
        success: true,
        message: format!("Membership updated to {:?}", update_req.new_tier),
    }))
}