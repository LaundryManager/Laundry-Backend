use argon2::{self, Config};

pub fn hash_password(password: String) -> String {
    // -- Put salt as a env variable
    let salt = "salt_for_laundry_app".as_bytes();
    let config = Config::default();
    let password = argon2::hash_raw(password.as_bytes(), salt, &config).unwrap();
    password.iter().map(|x| format!("{:02x}", x)).collect()
}