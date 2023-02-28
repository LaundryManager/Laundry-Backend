mod models;
mod utils;
mod database;
mod handlers;
mod routes;
mod configs;
use actix_web::{HttpServer, App, web::Data};
use database::connection::Datab;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = Data::new(Datab::init().await.expect("Error connecting to database"));
    let token_secret = Data::new(configs::configs::Settings::development().expect("Failed to read settings"));
    HttpServer::new(move || {
        App::new()
            .app_data(surreal.clone())
            .app_data(token_secret.clone())
            .service(routes::schedules::schedule_scope())
            .service(routes::user::user_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
