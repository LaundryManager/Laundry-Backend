mod models;
mod tools;
mod database;
use actix_web::http::StatusCode;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use database::connection::{create_user, verify_password};
use tools::hash_password::hash_password;
#[allow(dead_code)]

// -- Admin only
#[get("/all")]
async fn all_users() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/register")]
async fn register(req_body: Json<models::user::Tenant>) -> impl Responder {
    let desserialized_data = req_body.into_inner();
    let new_user = models::user::Tenant::new(desserialized_data.login, hash_password(desserialized_data.password), desserialized_data.apartment, desserialized_data.floor);

    match create_user(new_user).await {
         Ok(true) => {
            dbg!("User created");
            HttpResponse::Ok().status(StatusCode::CREATED).body("User created")
        },
         Ok(false) => {
            dbg!("User not created");
            HttpResponse::Ok().status(StatusCode::CONFLICT).body("Email already in use")
         }
         Err(_) => HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).body("User not created"),
    }

    //HttpResponse::Ok()
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
