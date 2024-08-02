use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/submit_analysis", web::post().to(handlers::submit_analysis))
            // Add other routes as needed
    );
}