use actix_web::{App, HttpServer, web};
use std::path::PathBuf;

use crate::api::HBankCommunication;
use crate::svrapp_execution::{ExecutionEngine, VMFactory};
use crate::svrapp_endpoints::configure_api;

mod api;
mod svrapp_execution;
mod svrapp_endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hbank_comm = web::Data::new(HBankCommunication::new(
        "your_server_app_id".to_string(),
        "http://hbank_url".to_string(),
    ));

    let vm_factory = VMFactory::new(
        PathBuf::from("/path/to/base/image"),
        "firecracker".to_string(), // or "docker"
    );
    let execution_engine = web::Data::new(ExecutionEngine::new(vm_factory));

    HttpServer::new(move || {
        App::new()
            .app_data(hbank_comm.clone())
            .app_data(execution_engine.clone())
            .configure(configure_api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}