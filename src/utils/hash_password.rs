use argon2::{self, Config};

pub fn hash_password(password: String, salt: String) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}