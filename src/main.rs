use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;
use dotenv::dotenv;
use env_logger::Env;

#[derive(Deserialize)]
struct ProjectCreateRequest {
    client_id: String,
    title: String,
}

#[derive(Serialize)]
struct ProjectResponse {
    project_id: String,
    message: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: i64,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().timestamp(),
    })
}

async fn create_project(
    item: web::Json<ProjectCreateRequest>,
) -> HttpResponse {
    let project_id = Uuid::new_v4().to_string();
    
    log::info!(
        "Creating project '{}' for client '{}' with ID: {}",
        item.title,
        item.client_id,
        project_id
    );

    HttpResponse::Ok().json(ProjectResponse {
        project_id,
        message: "Project created successfully".to_string(),
    })
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "name": "Workanda API",
        "version": "0.1.0",
        "description": "The Future of Freelancing – Secure, Transparent, and Empowering",
        "endpoints": {
            "health": "/health",
            "create_project": "POST /projects"
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("Starting Workanda API server on 0.0.0.0:5000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/projects", web::post().to(create_project))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
