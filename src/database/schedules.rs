use super::connection::*;
use actix_web::web::Data;
use crate::models::schedules::*;
use crate::database::connection::Datab;

pub async fn add_schedule(query_data: ScheduleReq, conn: Data<Datab>, user: String) -> Result<SchedulesAgenda, SchedulesError> {    
    let sql = format!("CREATE agenda SET order = {}", query_data.order);
    
    let mut query = match conn.connection.query(sql).await {
        Ok(result) => {
            dbg!(&result);
            result
        }
        Err(errinho) => {
            dbg!(errinho);
            return Err(SchedulesError::InvalidInformations);
        },
    };

    let agenda_id: Option<SchedulesQuery> = match query.take(0) {
        Ok(id) => {
            dbg!(&id);
            id
        },
        Err(errinho) => {
            dbg!(errinho);
            return Err(SchedulesError::InvalidInformations);
        }
    };

    let relation_query = format!("RELATE tenant:`{}`->using->{}", user, agenda_id.unwrap().id);

    let relation_maded = match conn.connection.query(relation_query).await {
        Ok(result) => {
            dbg!(&result);
            result
        }
        Err(errinho) => {
            dbg!(errinho);
            return Err(SchedulesError::InvalidInformations);
        },
    };

    Ok(SchedulesAgenda::Sixth)
}
