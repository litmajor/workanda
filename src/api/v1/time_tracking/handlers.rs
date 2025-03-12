use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::time_entry::TimeEntry;
use crate::models::timesheet::Timesheet;
use crate::models::time_tracking_report::TimeTrackingReport;

// Create a new time entry
pub async fn create_time_entry_handler(
    pool: web::Data<PgPool>,
    time_entry: web::Json<TimeEntry>,
) -> Result<HttpResponse, ApiError> {
    let new_entry = sqlx::query_as::<_, TimeEntry>(
        r#"
        INSERT INTO time_entries (user_id, task_id, start_time, end_time)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(time_entry.user_id)
    .bind(time_entry.task_id)
    .bind(time_entry.start_time)
    .bind(time_entry.end_time)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(new_entry))
}

// Get all time entries for a user
pub async fn get_time_entries_handler(
    pool: web::Data<PgPool>,
    web::Path(user_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let entries = sqlx::query_as::<_, TimeEntry>(
        "SELECT * FROM time_entries WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(entries))
}

// Generate a timesheet for a user
pub async fn generate_timesheet_handler(
    pool: web::Data<PgPool>,
    web::Path(user_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let entries = sqlx::query_as::<_, TimeEntry>(
        "SELECT * FROM time_entries WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool.as_ref())
    .await?;

    let mut timesheet = Timesheet::new(user_id);
    for entry in entries {
        timesheet.add_time_entry(entry);
    }

    Ok(HttpResponse::Ok().json(timesheet))
}

// Generate a time tracking report for a user
pub async fn generate_time_tracking_report_handler(
    pool: web::Data<PgPool>,
    web::Path(user_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let sheets = sqlx::query_as::<_, Timesheet>(
        r#"
        SELECT 
            user_id, 
            array_agg(row(time_entries.id, time_entries.user_id, time_entries.task_id, time_entries.start_time, time_entries.end_time)) AS time_entries
        FROM time_entries
        WHERE user_id = $1
        GROUP BY user_id
        "#,
    )
    .bind(user_id)
    .fetch_all(pool.as_ref())
    .await?;

    let mut report = TimeTrackingReport::new(user_id);
    for sheet in sheets {
        report.add_timesheet(sheet);
    }

    Ok(HttpResponse::Ok().body(report.generate_report()))
}