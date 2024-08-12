use actix_web::{App, HttpServer};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use h_bank::api::{
    public_api::configure_app,
    data_manager::DataManager,
    execution_engine::ExecutionEngine,
    archive_system::ArchiveSystem,
    result_processor::ResultProcessor,
};

// Assuming AppState and JobStatus are defined in one of the api submodules
use h_bank::api::models::{AppState, JobStatus};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data_manager = Arc::new(DataManager::new());
    let execution_engine = Arc::new(Mutex::new(ExecutionEngine::new()));
    let archive_system = Arc::new(ArchiveSystem::new());
    let result_processor = Arc::new(ResultProcessor::new());
    let jobs = Arc::new(Mutex::new(HashMap::new()));

    let state = actix_web::web::Data::new(AppState {
        data_manager,
        execution_engine,
        archive_system,
        result_processor,
        jobs,
    });

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure_app)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}