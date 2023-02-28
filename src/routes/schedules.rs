use actix_web::{Scope, HttpResponse, Responder, HttpRequest, web, web::Json, web::Data, FromRequest};
use crate::models::schedules::*;

pub fn schedule_scope() -> Scope {
    web::scope("/agenda")
        .route("/new", web::post().to(new_schedule))
}

pub async fn new_schedule(requisition: Json<ScheduleReq>) -> impl Responder {
    dbg!(requisition);

     HttpResponse::Ok()
}
