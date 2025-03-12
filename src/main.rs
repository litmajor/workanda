use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use rdkafka::{ClientConfig, producer::{FutureProducer, FutureRecord}};
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;

#[derive(Serialize)]
struct ProjectCreateRequest {
    client_id: String,
    title: String,
}

#[derive(Serialize)]
struct ProjectResponse {
    project_id: String,
}

#[derive(Serialize)]
struct ProjectCreated {
    project_id: String,
    client_id: String,
    title: String,
    created_at: i64,
}

async fn create_project(
    producer: web::Data<Arc<Mutex<FutureProducer>>>,
    item: web::Json<ProjectCreateRequest>,
) -> impl Responder {
    let project_id = Uuid::new_v4().to_string();
    
    // Create event
    let event = ProjectCreated {
        project_id: project_id.clone(),
        client_id: item.client_id.clone(),
        title: item.title.clone(),
        created_at: Utc::now().timestamp(),
    };

    // Serialize event to bytes
    let bytes = serde_json::to_vec(&event).unwrap();
    let record = FutureRecord::to("projects.created")
        .key(&project_id)
        .payload(&bytes);

    // Send event to Kafka
    let producer = producer.lock().await;
    producer.send(record, Duration::from_secs(3)).await.unwrap();

    // Return response
    HttpResponse::Ok().json(ProjectResponse { project_id })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let producer = Arc::new(Mutex::new(producer));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(producer.clone()))
            .route("/projects", web::post().to(create_project))
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
