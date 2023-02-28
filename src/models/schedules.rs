use crate::models::user::Tenant;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Object;
use anyhow::anyhow;
use serde_json;
#[allow(dead_code)]

pub struct Schedules {
    user: Tenant,
    agenda: SchedulesAgenda,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct SchedulesQuery {
    pub id: String,
    pub order: i8,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct ScheduleReq {
    pub order: i8,
}

pub enum SchedulesError {
    AlreadyInUse,
    TooMuchTries,
    InvalidInformations,
}

pub enum SchedulesAgenda {
    Error,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
}

impl SchedulesAgenda {
    fn number_to_enum(number: i32) -> SchedulesAgenda {
        match number {
            1 => SchedulesAgenda::First,
            2 => SchedulesAgenda::Second,
            3 => SchedulesAgenda::Third,
            4 => SchedulesAgenda::Fourth,
            5 => SchedulesAgenda::Fifth,
            6 => SchedulesAgenda::Sixth,
            7 => SchedulesAgenda::Seventh,
            8 => SchedulesAgenda::Eighth,
            9 => SchedulesAgenda::Ninth,
            10 => SchedulesAgenda::Tenth,
            _ => SchedulesAgenda::Error,
        }
    }
    fn return_hour(&self) -> String {
        match self {
            SchedulesAgenda::First => "7:00".into(),
            SchedulesAgenda::Second => "8:30".into(),
            SchedulesAgenda::Third => "10:00".into(),
            SchedulesAgenda::Fourth => "11:30".into(),
            SchedulesAgenda::Fifth => "13:00".into(),
            SchedulesAgenda::Sixth => "14:30".into(),
            SchedulesAgenda::Seventh => "16:00".into(),
            SchedulesAgenda::Eighth => "17:30".into(),
            SchedulesAgenda::Ninth => "19:00".into(),
            SchedulesAgenda::Tenth => "20:30".into(),
            _ => "Error".into(),
        }
    }
}

impl TryFrom<Object> for Schedules {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let user: Tenant = {
            serde_json::from_str(value.get("user").map(|x| x.to_string()).ok_or_else(|| anyhow!("no user"))?.replace('"', "").as_str()).unwrap()
        };
        let agenda = SchedulesAgenda::number_to_enum(value.get("time").map(|x| x.to_number().to_int()).ok_or_else(|| anyhow!("no time"))? as i32);
        dbg!(user.clone());
        Ok(Schedules {
            user: Tenant { id: "test".into(), password: "test".into(), apartment: 2, floor: 2 },
            agenda,
        })
    }
}

impl TryFrom<Object> for ScheduleReq {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let order = value.get("order").map(|x| x.to_number().to_int()).ok_or_else(|| anyhow!("no order"))? as i8;
    
        Ok(ScheduleReq { order })
    }
}
