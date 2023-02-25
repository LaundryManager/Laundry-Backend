#![allow(dead_code)]

use crate::configs::configs::Settings;
use crate::database::connection::Datab;
use crate::database::user::create_claims_from_login;
use crate::{models, database};
use actix_web::http::StatusCode;
use actix_web::{Scope, HttpResponse, Responder, web, web::Json, web::Data};
use jsonwebtoken::{encode, Header, EncodingKey};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
}

async fn register(req_body: Json<models::user::Tenant>, conn: Data<Datab>, settings: Data<Settings>) -> impl Responder {
    match database::user::create_user(req_body.into_inner(), conn, &settings.secret.password_salt).await {
        Ok(true) => {
            HttpResponse::Ok().status(StatusCode::CREATED).body("User created")
        },
        Ok(false) => {HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).body("Try again later")},
        Err(_) => {HttpResponse::Ok().status(StatusCode::NOT_ACCEPTABLE).body("Invalid user data")},
    }
}

async fn login(req_body: Json<models::user::Login>, conn: Data<Datab>, settings: Data<Settings>) -> impl Responder {
    let logins = req_body.into_inner();
    match create_claims_from_login(logins, conn).await {
        Ok(claims) => {
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(settings.secret.jwt_secret.as_ref())).unwrap();
            HttpResponse::Ok().status(StatusCode::CREATED).body(token)
        },
        Err(_) => {
            HttpResponse::Ok().status(StatusCode::NOT_ACCEPTABLE).body("Invalid user data")},
    }
}