use config::{Config, File, ConfigError,};
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Secrets {
    pub jwt_secret: String,
    pub password_salt: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub url: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub secret:  Secrets,
}

// Extract AppConfig
impl Settings {
    pub fn development() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("src/configs/development.toml"));

        match settings.build() {
            Ok(app_config) => app_config.try_deserialize(),
            Err(e) => panic!("Error: {}", e),
        }
    }
}