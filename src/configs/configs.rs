use config::{Config, File, ConfigError,};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub ns: String,
    pub db: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
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
