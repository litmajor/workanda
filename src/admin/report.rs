// src/admin/report.rs
use sqlx::PgPool;
use csv::Writer;
use std::io::Cursor;

pub async fn generate_financial_report(
    pool: &PgPool,
    current_user: &User,
) -> Result<String, String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    let total_escrow = sqlx::query!(
        "SELECT SUM(amount) AS total FROM escrow_accounts"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?
    .total
    .unwrap_or(0.0);

    Ok(format!("Total Escrow Funds: ${}", total_escrow))
}

pub async fn get_activity_logs(
    pool: &PgPool,
    current_user: &User,
) -> Result<Vec<ActivityLog>, String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    sqlx::query_as!(
        ActivityLog,
        r#"
        SELECT * FROM activity_logs
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}

pub async fn generate_revenue_report(
    pool: &PgPool,
) -> Result<Vec<RevenueByTier>, String> {
    let revenue_by_tier = sqlx::query_as!(
        RevenueByTier,
        r#"
        SELECT mt.name AS tier_name, COALESCE(SUM(t.amount), 0.0) AS total_revenue
        FROM membership_tiers mt
        LEFT JOIN user_memberships um ON mt.id = um.tier_id
        LEFT JOIN transactions t ON um.id = t.user_membership_id
        GROUP BY mt.name
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(revenue_by_tier)
}

pub async fn generate_user_growth_report(
    pool: &PgPool,
) -> Result<(usize, Vec<NewUser>, Vec<ActiveUsersOverTime>), String> {
    let total_users = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or(0);

    let new_users = sqlx::query_as!(
        NewUser,
        r#"
        SELECT id, username, created_at AS "registration_date!: DateTime<Utc>"
        FROM users
        ORDER BY created_at DESC
        LIMIT 10
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let active_users_over_time = sqlx::query_as!(
        ActiveUsersOverTime,
        r#"
        SELECT DATE_TRUNC('day', last_active) AS "date!: DateTime<Utc>", COUNT(user_id) AS active_users
        FROM user_sessions
        GROUP BY DATE_TRUNC('day', last_active)
        ORDER BY date ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((total_users, new_users, active_users_over_time))
}


pub async fn export_revenue_report(
    pool: &PgPool,
) -> Result<HttpResponse, String> {
    let revenue_by_tier = generate_revenue_report(pool).await?;

    let mut csv_data = Cursor::new(Vec::new());
    let mut writer = Writer::from_writer(&mut csv_data);
    writer.write_record(&["Tier Name", "Total Revenue"])?;

    for tier in revenue_by_tier {
        writer.write_record(&[tier.tier_name, format!("{}", tier.total_revenue)])?;
    }

    writer.flush()?;
    let csv_content = String::from_utf8(csv_data.into_inner()).map_err(|e| e.to_string())?;

    Ok(HttpResponse::Ok()
        .content_type("text/csv")
        .header("Content-Disposition", "attachment; filename=\"revenue_report.csv\"")
        .body(csv_content))
}


#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    pub report_type: String, // e.g., "revenue", "user_growth"
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn generate_custom_report(
    report_request: web::Json<ReportRequest>,
    db: web::Data<Pool>,
    req: HttpRequest,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap();

    if claims.role != UserRole::Admin {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let conn = db.get().expect("Couldn't get db connection");

    match report_request.report_type.as_str() {
        "revenue" => {
            let revenue_stats = fetch_revenue_stats(&conn, report_request.start_date, report_request.end_date).await;
            HttpResponse::Ok().json(revenue_stats)
        }
        "user_growth" => {
            let user_growth = fetch_user_growth(&conn, report_request.start_date, report_request.end_date).await;
            HttpResponse::Ok().json(user_growth)
        }
        _ => HttpResponse::BadRequest().body("Invalid report type"),
    }
}

pub async fn export_report(
    report_request: web::Json<ReportRequest>,
    db: web::Data<Pool>,
    req: HttpRequest,
) -> impl Responder {
    let claims = req.extensions().get::<Claims>().unwrap();

    if claims.role != UserRole::Admin {
        return HttpResponse::Forbidden().body("Access denied");
    }

    let conn = db.get().expect("Couldn't get db connection");

    match report_request.report_type.as_str() {
        "revenue" => {
            let csv_data = generate_revenue_csv(&conn, report_request.start_date, report_request.end_date).await;
            HttpResponse::Ok()
                .content_type("text/csv")
                .header("Content-Disposition", "attachment; filename=\"revenue_report.csv\"")
                .body(csv_data)
        }
        "user_growth" => {
            let csv_data = generate_user_growth_csv(&conn, report_request.start_date, report_request.end_date).await;
            HttpResponse::Ok()
                .content_type("text/csv")
                .header("Content-Disposition", "attachment; filename=\"user_growth_report.csv\"")
                .body(csv_data)
        }
        _ => HttpResponse::BadRequest().body("Invalid report type"),
    }
}