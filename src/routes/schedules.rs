use actix_web::{Scope, HttpResponse, Responder, HttpRequest, web, web::Json, web::Data};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/all", web::get().to(all_users))
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
}


