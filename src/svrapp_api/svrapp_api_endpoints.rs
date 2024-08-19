use actix_web::{web, HttpResponse, Responder};
use super::HBankCommunication;
use super::shared_models::*;

pub async fn submit_code(
    submission: web::Json<CodeSubmission>,
    hbank_comm: web::Data<HBankCommunication>,
) -> impl Responder {
    match hbank_comm.submit_code(submission.into_inner()) {
        Ok(job_id) => HttpResponse::Ok().json(job_id),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_cohort_info(
    cohort_id: web::Path<String>,
    hbank_comm: web::Data<HBankCommunication>,
) -> impl Responder {
    match hbank_comm.get_cohort_info(&cohort_id) {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// Add other endpoint handlers...

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/submit_code", web::post().to(submit_code))
            .route("/cohort/{cohort_id}", web::get().to(get_cohort_info))
        // Add other routes...
    );
}