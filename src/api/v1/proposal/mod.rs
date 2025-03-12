pub mod routes;
pub mod models;
pub mod submit;
pub mod review;

pub use self::routes::*;
pub use self::models::*;

use actix_web::web;
use crate::api::proposal::{submit, review};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/proposal/submit")
            .route(web::post().to(submit::submit_proposal_handler)),
    );
    cfg.service(
        web::resource("/proposal/review")
            .route(web::post().to(review::review_proposal_handler)),
    );
}
