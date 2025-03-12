use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use workanda_models::User;
use sqlx::postgres::PgPoolOptions;
use crate::user::{routes::config, middleware::validator};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://workanda:workandaforever@localhost/workanda")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .app_data(web::Data::new(auth_service))
    .configure(config)
    .wrap(
        Authentication::new(validator)
            .realm("Protected area")
            .scope("api")
    )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

