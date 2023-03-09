use actix_web::web::Data;
use crate::models::schedules::*;
use crate::database::connection::Datab;
use crate::handlers::jwt_validation_handler::AuthenticationToken;
use chrono::{Utc, Datelike};

pub async fn add_schedule(query_data: ScheduleReq, conn: Data<Datab>, user: &String, user_token: &AuthenticationToken) -> Result<SchedulesAgenda, SchedulesError> {    
    let sql = format!("CREATE agenda SET order = {}", query_data.order);
    
    // TODO: Validate if the user can register -> Floor 3 is friday, for example
    // TODO: Validate if the user is registering many times
    
    let date_time = Utc::now();
    let weekday = date_time.weekday();

    match weekday {
        chrono::Weekday::Mon => {
            if user_token.id.floor != 1 {
                return Err(SchedulesError::RestrictedDateError);
            }
        },
        chrono::Weekday::Tue => {
            if user_token.id.floor != 2 {
                return Err(SchedulesError::RestrictedDateError);
            }
        },
        chrono::Weekday::Wed => {
            if user_token.id.floor != 2 {
                return Err(SchedulesError::RestrictedDateError);
            }
        },
        chrono::Weekday::Thu => {
            if user_token.id.floor != 3 {
                return Err(SchedulesError::RestrictedDateError);
            }
        },
        chrono::Weekday::Fri => {
            if user_token.id.floor != 3 {
                return Err(SchedulesError::RestrictedDateError);
            }
        },
        chrono::Weekday::Sat => {
            if user_token.id.floor != 4 {
                return Err(SchedulesError::RestrictedDateError);
            }
        },
        chrono::Weekday::Sun => {
            if user_token.id.floor != 4 {
                return Err(SchedulesError::RestrictedDateError);
            }
        },
    }

    dbg!(weekday);

    dbg!(user_token.id.floor);
    let mut query = match conn.connection.query(sql).await {
        Ok(result) => {
            result
        }
        Err(_) => {
            return Err(SchedulesError::InvalidInformations);
        },
    };

    let agenda_id: Option<SchedulesQuery> = match query.take(0) {
        Ok(id) => {
            id
        },
        Err(_) => {
            return Err(SchedulesError::InvalidInformations);
        }
    };

    let relation_query = format!("RELATE tenant:`{}`->using->{}", user, agenda_id.clone().unwrap().id);

    match conn.connection.query(relation_query).await {
        Ok(_) => {
            return Ok(SchedulesAgenda::number_to_enum(
                agenda_id.unwrap().order as i32
            )); 
        },
        Err(_) => {
            return Err(SchedulesError::InvalidInformations);
        },
    }
}

pub async fn all_today_schedules(conn: Data<Datab>) -> Result<Vec<Orders>, SchedulesError> {
    // what happens if the vec is empty?
    let query_data = conn.connection
    .query("SELECT id as user, ->using->agenda.order as orders FROM tenant WHERE count(->using->agenda) > 0").await;

    let test: Vec<Orders> = match query_data.unwrap().take(0) {
        Ok(result) => {
            result
        },
        Err(_) => {
            return Err(SchedulesError::InvalidInformations);
        },
    };
    Ok(Orders::return_formated(test))
}