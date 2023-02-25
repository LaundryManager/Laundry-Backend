use serde::{Deserialize, Serialize};
use surrealdb::sql::Object;
use anyhow::anyhow;

#[derive(Deserialize, Debug, Clone)]
pub struct Login {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TenantClaims {
  pub login: String,
  pub apartment: i32,
  pub floor: i32,
  pub exp: usize,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tenant {
    pub id: String,
    pub password: String,
    pub apartment: i32,
    pub floor: i32,
}


impl Tenant {
    pub fn new(id: String, password: String, apartment: i32, floor: i32) -> Tenant {
        Tenant {
            id,
            password,
            apartment,
            floor,
        }
    }
}

impl TryFrom<Object> for Tenant {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let id = value.get("id").map(|x| x.to_string()).ok_or_else(|| anyhow!("no login"))?.replace('"', "");
        let password = value.get("password").map(|x| x.to_string()).ok_or_else(|| anyhow!("no password"))?.replace('"', "");
        let apartment = value.get("apartment").map(|x| x.to_number().to_int()).ok_or_else(|| anyhow!("no apartment"))?;
        let floor = value.get("floor").map(|x| x.to_number().to_int()).ok_or_else(|| anyhow!("no floor"))?;

        Ok(Tenant {
            id,
            password,
            apartment: apartment as i32,
            floor: floor as i32,
        })
    }
}
