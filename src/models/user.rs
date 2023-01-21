use argon2::{self, Config};
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Debug)]
pub struct Login {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub login: String,
    pub password: String,
    pub apartment: i32,
    pub floor: i32,
}

impl User {
    pub fn new(login: String, password: String, apartment: i32, floor: i32) -> User {
        User {
            login,
            password: hash_password(password, String::from("mahatmamahatma")),
            apartment,
            floor,
        }
    }
}

fn hash_password(password: String, salt: String) -> String {
    let config = Config::default();
    let password = argon2::hash_raw(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    password.iter().map(|x| format!("{:02x}", x)).collect()
}