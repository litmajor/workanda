pub mod dispute;
pub mod escalation;
pub mod payment;

use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/dispute/{escrow_id}")
            .route(web::post().to(dispute::handle_dispute)),
    );
    cfg.service(
        web::resource("/escalate/{dispute_id}")
            .route(web::post().to(escalation::escalate_dispute)),
    );
    cfg.service(
        web::resource("/send-reminders")
            .route(web::post().to(payment::send_payment_reminders)),
    );
}
