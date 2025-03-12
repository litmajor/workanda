// src/admin/handlers.rs
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use actix_web_actors::ws;


struct DashboardWs;

impl actix::Actor for DashboardWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for DashboardWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => ctx.stop(),
        }
    }
}

pub async fn dashboard_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(DashboardWs {}, &req, stream)
}


pub async fn get_admin_dashboard_data(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    let jobs_posted = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM jobs"
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or(0);

    let clients_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE role = 'Client'"
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or(0);

    let freelancers_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE role = 'Freelancer'"
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or(0);

    let active_projects_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM projects WHERE status = 'Active'"
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or(0);

    let membership_tiers = sqlx::query_as::<_, String>(
        "SELECT name FROM membership_tiers"
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?;

    let notifications = sqlx::query_as::<_, String>(
        "SELECT message FROM system_notifications ORDER BY created_at DESC LIMIT 10"
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?;

    transaction.commit().await.map_err(|e| e.to_string())?;

    let dashboard_data = super::models::AdminDashboardData {
        jobs_posted,
        clients_count,
        freelancers_count,
        active_projects_count,
        membership_tiers,
        notifications,
    };

    Ok(HttpResponse::Ok().json(dashboard_data))
}
pub async fn delete_user_data(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, String> {
    let claims = req.extensions().get::<Claims>().unwrap();

    if claims.role != UserRole::Admin {
        return Err("Access denied".to_string());
    }

    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    // Delete user data
    let rows_affected = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id.into_inner()
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?
    .rows_affected();

    if rows_affected == 0 {
        return Err("User not found".to_string());
    }

    // Log the action
    log_admin_action(&mut *transaction, claims.user_id, "Delete User Data", Some(format!("Deleted user ID: {}", user_id)))?;

    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(HttpResponse::Ok().body("User data deleted successfully"))
}

//bestowed admin creation ability
pub async fn create_client(
    db: web::Data<Pool>,
    new_client: web::Json<UserInput>,
    req: HttpRequest,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap();

    if claims.role != UserRole::Admin {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let conn = db.get().expect("Couldn't get db connection");

    let hashed_password = hash_password(&new_client.password).await.unwrap_or_else(|_| "INVALID_HASH".to_string());

    let user = User {
        id: 0,
        username: new_client.username.clone(),
        email: new_client.email.clone(),
        password_hash: hashed_password,
        role: UserRole::Client,
        membership_tier: MembershipTier::Normal,
        account_type: AccountType::Standard,
        location: Location::default(),
        subscription_status: SubscriptionStatus::Active,
        subscription_end_date: None,
        stripe_customer_id: None,
        privileges: Privileges::from_role(UserRole::Client),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match diesel::insert_into(users::table)
        .values(&user)
        .execute(&conn)
    {
        Ok(_) => {
            log_admin_action(&conn, claims.user_id, "Create Client", Some(&format!("Created client: {}", new_client.username))).unwrap_or_default();
            HttpResponse::Created().json(user)
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to create client"),
    }
}
pub async fn create_freelancer(
    db: web::Data<Pool>,
    new_freelancer: web::Json<UserInput>,
    req: HttpRequest,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap();

    if claims.role != UserRole::Admin {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let conn = db.get().expect("Couldn't get db connection");

    let hashed_password = hash_password(&new_freelancer.password).await.unwrap_or_else(|_| "INVALID_HASH".to_string());

    let user = User {
        id: 0,
        username: new_freelancer.username.clone(),
        email: new_freelancer.email.clone(),
        password_hash: hashed_password,
        role: UserRole::Freelancer,
        membership_tier: MembershipTier::Normal,
        account_type: AccountType::Standard,
        location: Location::default(),
        subscription_status: SubscriptionStatus::Active,
        subscription_end_date: None,
        stripe_customer_id: None,
        privileges: Privileges::from_role(UserRole::Freelancer),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match diesel::insert_into(users::table)
        .values(&user)
        .execute(&conn)
    {
        Ok(_) => {
            log_admin_action(&conn, claims.user_id, "Create Freelancer", Some(&format!("Created freelancer: {}", new_freelancer.username))).unwrap_or_default();
            HttpResponse::Created().json(user)
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to create freelancer"),
    }
}

pub async fn delete_client(
    db: web::Data<Pool>,
    client_id: web::Path<i32>,
    req: HttpRequest,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap();

    if claims.role != UserRole::Admin {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let conn = db.get().expect("Couldn't get db connection");

    let deleted = diesel::delete(users::table.filter(users::id.eq(*client_id)).filter(users::role.eq(UserRole::Client)))
        .execute(&conn)
        .unwrap_or(0);

    if deleted > 0 {
        log_admin_action(&conn, claims.user_id, "Delete Client", Some(&format!("Deleted client ID: {}", client_id))).unwrap_or_default();
        HttpResponse::Ok().body("Client deleted successfully")
    } else {
        HttpResponse::NotFound().body("Client not found")
    }
}