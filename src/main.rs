mod models;
mod utils;
mod database;
mod handlers;
mod routes;
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(routes::user::user_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
