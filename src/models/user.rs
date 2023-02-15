use serde::{Deserialize, Serialize};

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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tenant {
    pub login: String,
    pub password: String,
    pub apartment: i32,
    pub floor: i32,
}

impl Tenant {
    pub fn new(login: String, password: String, apartment: i32, floor: i32) -> Tenant {
        Tenant {
            login,
            password,
            apartment,
            floor,
        }
    }
}

