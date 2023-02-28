use actix_web::http::StatusCode;
use actix_web::{Scope, HttpResponse, Responder, HttpRequest, web, web::Json, web::Data, FromRequest};
use crate::models::schedules::*;
use crate::handlers::jwt_validation_handler::AuthenticationToken; 
use crate::database::schedules::*;
use crate::database::connection::Datab;

pub fn schedule_scope() -> Scope {
    web::scope("/agenda")
        .route("/new", web::post().to(new_schedule))
}

pub async fn new_schedule(requisition: Json<ScheduleReq>, user: AuthenticationToken, bddata: Data<Datab>) -> impl Responder {

    match add_schedule(requisition.into_inner(), bddata, user.id.login).await {
        Ok(_) => {
            HttpResponse::Ok().status(StatusCode::CREATED).body("Done!")
        },
        Err(_) => {
            HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).body("Do you have a valid JWT?")
        }
    }
}
