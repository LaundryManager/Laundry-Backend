mod models;
mod tools;
mod database;
use database::connection::{show_all, create_user, verify_password};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use tools::hash_password::hash_password;
#[allow(dead_code)]

// -- Admin only
#[get("/all")]
async fn all_users() -> impl Responder {
    show_all().await;
    HttpResponse::Ok().body("Hello world!")
}

#[post("/register")]
async fn register(req_body: Json<models::user::Tenant>) -> impl Responder {
    let desserialized_data = req_body.into_inner();
    let new_user = models::user::Tenant::new(desserialized_data.login, hash_password(desserialized_data.password), desserialized_data.apartment, desserialized_data.floor);

    // -- Add log files
    match create_user(new_user).await {
         Ok(_) => dbg!("User created"),
         Err(_) => dbg!("Error"),
    };

    HttpResponse::Ok()
}

#[post("/login")]
async fn login(req_body: Json<models::user::Login>) -> impl Responder {
    match verify_password(req_body.into_inner()).await {
        Ok(true) => HttpResponse::Ok().body("User verified"),
        Ok(false) => HttpResponse::Ok().body("Invalid username or password"),
        Err(_) => HttpResponse::Ok().body("Invalid username or password"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(register)
            .service(all_users)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
