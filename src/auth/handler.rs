use actix_web::{web, HttpResponse, Responder};

pub async fn protected_handler(
    req: web::HttpRequest,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|auth| auth.to_str().ok())
        .map(|s| s.strip_prefix("Bearer ").unwrap_or_default())
        .unwrap_or_default();

    match decode_jwt(token) {
        Ok(claims) => {
            info!("Authenticated user: {}", claims.sub);
            HttpResponse::Ok().body("Access granted")
        }
        Err(_) => HttpResponse::Unauthorized().body("Unauthorized"),
    }
}