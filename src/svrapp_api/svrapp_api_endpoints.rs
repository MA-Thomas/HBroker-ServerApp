use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::svrapp_api::svrapp_hbank_communication::HBankCommunication;
use crate::svrapp_api::svrapp_shared_models::{CodeSubmission, SyntheticDataSetup, AnalysisResult};
use crate::svrapp_execution::svrapp_engine::{ExecutionEngine, ExecutionEngineTrait};

pub async fn submit_code(
    submission: web::Json<CodeSubmission>,
    hbank_comm: web::Data<HBankCommunication>,
) -> impl Responder {
    match hbank_comm.submit_code(&submission).await {
        Ok(job_id) => HttpResponse::Ok().json(json!({ "job_id": job_id })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to submit code: {}", e)
        }))
    }
}

pub async fn start_job(
    job_id: web::Path<String>,
    hbank_comm: web::Data<HBankCommunication>,
    execution_engine: web::Data<ExecutionEngine>,
) -> impl Responder {
    // First, retrieve the code submission from HBank
    let submission = match hbank_comm.get_code_submission(&job_id).await {
        Ok(sub) => sub,
        Err(e) => return HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to retrieve code submission: {}", e)
        }))
    };

    // Then, queue the job for execution
    if let Err(e) = execution_engine.queue_job(&job_id, &submission).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to queue job: {}", e)
        }));
    }

    HttpResponse::Accepted().json(json!({
        "job_id": job_id.to_string(),
        "status": "queued"
    }))
}

pub async fn get_job_status(
    job_id: web::Path<String>,
    execution_engine: web::Data<ExecutionEngine>,
) -> impl Responder {
    match execution_engine.get_job_status(&job_id).await {
        Some(status) => HttpResponse::Ok().json(status),
        None => HttpResponse::NotFound().json(json!({"error": "Job not found"})),
    }
}

pub async fn get_cohort_info(
    cohort_id: web::Path<String>,
    hbank_comm: web::Data<HBankCommunication>,
) -> impl Responder {
    match hbank_comm.get_cohort_info(&cohort_id).await {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to retrieve cohort info: {}", e)
        }))
    }
}

pub async fn setup_synthetic_data(
    setup: web::Json<SyntheticDataSetup>,
    hbank_comm: web::Data<HBankCommunication>,
) -> impl Responder {
    match hbank_comm.setup_synthetic_data(&setup).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to setup synthetic data: {}", e)
        }))
    }
}

pub async fn submit_analysis_result(
    result: web::Json<AnalysisResult>,
    hbank_comm: web::Data<HBankCommunication>,
) -> impl Responder {
    match hbank_comm.submit_analysis_result(&result).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "result submitted successfully" })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to submit analysis result: {}", e)
        }))
    }
}

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/submit_code", web::post().to(submit_code))
            .route("/job/{job_id}/start", web::post().to(start_job))
            .route("/job/{job_id}/status", web::get().to(get_job_status))
            .route("/cohort/{cohort_id}", web::get().to(get_cohort_info))
            .route("/synthetic/setup", web::post().to(setup_synthetic_data))
            .route("/result", web::post().to(submit_analysis_result))
    );
}