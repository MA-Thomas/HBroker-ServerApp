use actix_web::{App, HttpServer};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use h_bank::contracts::cohorts::public_api::{
    AppState, configure_app, DataManager, ExecutionEngine, ArchiveSystem, ResultProcessor, JobStatus
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize components
    let data_manager = Arc::new(DataManager::new());
    let execution_engine = Arc::new(Mutex::new(ExecutionEngine::new()));
    let archive_system = Arc::new(ArchiveSystem::new());
    let result_processor = Arc::new(ResultProcessor::new());
    let jobs = Arc::new(Mutex::new(HashMap::new()));

    // Create shared state
    let state = actix_web::web::Data::new(AppState {
        data_manager,
        execution_engine,
        archive_system,
        result_processor,
        jobs,
    });

    // Set up and run the server
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