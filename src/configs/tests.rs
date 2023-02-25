#[cfg(test)]mod tests {
    use super::super::configs::*;
    use std::error::Error;
    
    #[test]
    fn develop_settings_import() -> Result<(), Box<dyn Error>> {
        let dev_settings = Settings {
                server: ServerConfig {
                host: "127.0.0.1".to_owned(),
                port: 8080
            },
            database: DatabaseConfig {
                username: "root".to_owned(),
                password: "root".to_owned(),
                url: "localhost:8000".to_owned(),
                namespace: "namespace".to_owned(),
                database: "database".to_owned(),
            },
            secret: Secrets {
                jwt_secret: "ilovemila".to_owned(),
                password_salt: "supersafekids".to_owned(),
            },
        };
        assert_eq!(dev_settings, Settings::development()?);
        Ok(())
    }

}
