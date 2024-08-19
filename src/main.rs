use actix_web::{App, HttpServer};
use hbroker_server_app::svrapp_api::{HBankCommunication, configure_api};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hbank_comm = web::Data::new(HBankCommunication::new());

    HttpServer::new(move || {
        App::new()
            .app_data(hbank_comm.clone())
            .configure(configure_api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
