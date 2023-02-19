use argon2::{self, Config};

pub fn hash_password(password: String) -> String {
    let salt = "salt_for_laundry_app".as_bytes();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}