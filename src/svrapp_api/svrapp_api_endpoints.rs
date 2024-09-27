use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::svrapp_api::svrapp_hbank_communication::HBankCommunication;
use crate::svrapp_api::shared_models::{CodeSubmission, ExecutionMode, AnalysisResult};
use crate::svrapp_execution::svrapp_engine::{ExecutionEngine, ExecutionEngineTrait};

pub async fn submit_code(
    submission: web::Json<CodeSubmission>,
    hbank_comm: web::Data<HBankCommunication>,
    execution_engine: web::Data<ExecutionEngine>,
) -> impl Responder {
    if !validate_wasm_binary(&submission.wasm_code) {
        return HttpResponse::BadRequest().json(json!({
            "error": "Invalid WASM binary"
        }));
    }

    match submission.execution_mode {
        ExecutionMode::Local => {
            match execution_engine.queue_job(&submission).await {
                Ok(job_id) => HttpResponse::Ok().json(json!({ "job_id": job_id, "mode": "local" })),
                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to queue local job: {}", e)
                }))
            }
        },
        ExecutionMode::Remote => {
            match hbank_comm.submit_wasm_for_remote_execution(&submission).await {
                Ok(job_id) => HttpResponse::Ok().json(json!({ "job_id": job_id, "mode": "remote" })),
                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to submit remote job: {}", e)
                }))
            }
        }
    }
}
fn validate_wasm_binary(wasm_code: &[u8]) -> bool {
    // Implement WASM binary validation logic
    // This could include checking for WASM magic numbers, validating structure, etc.
    // For now, we'll just check if it starts with the WASM magic number
    wasm_code.starts_with(&[0x00, 0x61, 0x73, 0x6D])
}


pub async fn get_job_status(
    job_id: web::Path<String>,
    execution_engine: web::Data<ExecutionEngine>,
    hbank_comm: web::Data<HBankCommunication>,
) -> impl Responder {
    // First, check if it's a local job
    if let Some(status) = execution_engine.get_job_status(&job_id).await {
        return HttpResponse::Ok().json(json!({ "status": status, "mode": "local" }));
    }

    // If not found locally, check with HBank
    match hbank_comm.get_remote_execution_result(&job_id).await {
        Ok(result) => HttpResponse::Ok().json(json!({ "status": result.status, "mode": "remote" })),
        Err(_) => HttpResponse::NotFound().json(json!({ "error": "Job not found" })),
    }
}

//--------

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