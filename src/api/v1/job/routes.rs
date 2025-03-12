use sqlx::PgPool;
use crate::models::job::NewJob;
use crate::models::job::Job;
use actix_web::web;


pub async fn create_job(
    pool: web::Data<PgPool>,
    job: web::Json<NewJob>,
) -> Result<HttpResponse, ApiError> {
    let new_job = job.into_inner();

    let created_job = sqlx::query_as::<_, Job>(
        r#"
        INSERT INTO jobs (title, description, budget, deadline, client_id, category, priority)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(&new_job.title)
    .bind(&new_job.description)
    .bind(new_job.budget)
    .bind(new_job.deadline)
    .bind(new_job.client_id)
    .bind(&new_job.category)
    .bind(new_job.priority)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Job created successfully: {:?}", created_job);
    Ok(HttpResponse::Created().json(created_job))
}

pub async fn get_jobs() -> impl Responder {
    match get_all_jobs_from_db().await {
        Ok(jobs) => {
            info!("Retrieved all jobs successfully");
            HttpResponse::Ok().json(jobs)
        }
        Err(e) => {
            error!("Failed to retrieve jobs: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}


pub async fn get_jobs(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let jobs = sqlx::query_as::<_, Job>("SELECT * FROM jobs")
        .fetch_all(pool.as_ref())
        .await?;

    info!("Retrieved all jobs successfully");
    Ok(HttpResponse::Ok().json(jobs))
}

pub async fn get_job(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.as_ref())
        .await?
        .ok_or(ApiError::NotFound("Job not found".to_string()))?;

    info!("Retrieved job with ID {} successfully: {:?}", id, job);
    Ok(HttpResponse::Ok().json(job))
}

pub async fn update_job(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<u32>,
    job: web::Json<Job>,
) -> Result<HttpResponse, ApiError> {
    let updated_job = job.into_inner();

    let job = sqlx::query_as::<_, Job>(
        r#"
        UPDATE jobs
        SET title = COALESCE($2, title),
            description = COALESCE($3, description),
            budget = COALESCE($4, budget),
            deadline = COALESCE($5, deadline),
            category = COALESCE($6, category),
            priority = COALESCE($7, priority)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&updated_job.title)
    .bind(&updated_job.description)
    .bind(updated_job.budget)
    .bind(updated_job.deadline)
    .bind(&updated_job.category)
    .bind(updated_job.priority)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Updated job with ID {} successfully: {:?}", id, job);
    Ok(HttpResponse::Ok().json(job))
}

pub async fn delete_job(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query("DELETE FROM jobs WHERE id = $1")
        .bind(id)
        .execute(pool.as_ref())
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound("Job not found".to_string()));
    }

    info!("Deleted job with ID {} successfully", id);
    Ok(HttpResponse::Ok().body("Job deleted successfully"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/jobs")
            .route("", web::post().to(create_job_handler))
            .route("/{id}", web::get().to(get_job_handler))
            .route("/{id}", web::put().to(update_job_handler))
            .route("/{id}", web::delete().to(delete_job_handler)),
    );
}