use actix_web::http::StatusCode;
use actix_web::{Scope, HttpResponse, Responder, web, web::Json, web::Data};
use crate::models::schedules::*;
use crate::handlers::jwt_validation_handler::AuthenticationToken; 
use crate::database::schedules::*;
use crate::database::connection::Datab;

pub fn schedule_scope() -> Scope {
    web::scope("/agenda")
        .route("/new", web::post().to(new_schedule))
}

pub async fn new_schedule(requisition: Json<ScheduleReq>, user: AuthenticationToken, bddata: Data<Datab>) -> impl Responder {

    match add_schedule(requisition.into_inner(), bddata, &user.id.login, &user).await {
        Ok(tete) => {
            HttpResponse::Ok().status(StatusCode::CREATED)
                .body(format!("You can use at {}", tete.return_hour()))
        },
        Err(error_kind) => {
            match error_kind {
                SchedulesError::RestrictedDateError => {
                    HttpResponse::Ok().status(StatusCode::FORBIDDEN).body("You can't register at this day!")
                },
                SchedulesError::InvalidInformations => {
                    HttpResponse::Ok().status(StatusCode::BAD_REQUEST).body("Invalid informations!")
                },
                _ => {
                HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).body("Contact admin for more help!")
                }
            }
        }
    }
}
