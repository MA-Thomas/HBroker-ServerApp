use h_bank::contracts::cohorts::*; 
use h_bank::cohort_api;

mod api;
mod data_management;
mod sandbox;
mod script_execution;
mod result_processing;
mod security;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(api::routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



fn some_function() {
    let contract_info = cohort_api::get_contract_info("cohort1");
    let summary = cohort_api::get_cohort_summary("cohort1");
    // ... and so on
}