use actix_web::{App, HttpServer, web};
use hbroker_server_app::svrapp_api::{HBankCommunication, configure_api};
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let base_data_path = PathBuf::from("/path/to/your/data"); // Adjust this path as needed
    let hbank_comm = web::Data::new(HBankCommunication::new(base_data_path));

    HttpServer::new(move || {
        App::new()
            .app_data(hbank_comm.clone())
            .configure(configure_api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
