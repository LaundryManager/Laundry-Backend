use handlers::jwt_validation_handler::AuthenticationToken;
use actix_web::http::StatusCode;
use actix_web::{Scope, HttpResponse, Responder, HttpRequest, web};
use actix_web::web::Json;
use database::connection::{create_user, verify_password, get_user_claims};
use utils::hash_password::hash_password;
use jsonwebtoken::{
    encode,
    Header,
    EncodingKey,
};

use crate::handlers;
use crate::utils;
use crate::database;
use crate::models;


pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/all", web::get().to(all_users))
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
}

async fn all_users(_request: HttpRequest, auth_token: AuthenticationToken) -> impl Responder {
    dbg!(auth_token);
    HttpResponse::Ok().body("Authorized")
}

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
}

async fn login(req_body: Json<models::user::Login>) -> impl Responder {
    let login_struct = req_body.into_inner();
    let login_value = login_struct.clone().login;
    match verify_password(login_struct).await {
        Ok(true) => {
            match get_user_claims(login_value).await {
                Ok(claims) => {
                    let jwt = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
                    HttpResponse::Ok().insert_header(("Authorization", format!("Bearer {}", jwt))).json(jwt)
                },
                Err(_) => {
                    HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).body("Username or Password invalid!")
                }
            }
        },
        Ok(false) => HttpResponse::Ok().body("Invalid username or password"),
        Err(_) => HttpResponse::Ok().body("Invalid username or password"),
    }
}