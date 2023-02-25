use argon2::{self, Config};

pub fn hash_password(password: String, salt: String) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

pub fn verify_password(password: String, hash: String) -> bool {
    argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
}