use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
mod models;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/register")]
async fn register(req_body: Json<models::User::User>) -> impl Responder {    
    let desserialized_data = req_body.into_inner();
    let new_user = models::User::User::new(desserialized_data.login, desserialized_data.password, desserialized_data.apartxxment, desserialized_data.floor);
    // Todo: add user to database
    dbg!(new_user);
    HttpResponse::Ok()
}

#[post("/login")]
async fn login(req_body: Json<models::User::Login>) -> impl Responder {
    dbg!(req_body);
    // Todo: check if user exists in database and return session token
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
