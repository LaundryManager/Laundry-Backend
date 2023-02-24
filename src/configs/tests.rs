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
                url: "file://develop.db".to_owned(),
                ns: "develop_ns".to_owned(),
                db: "develop_db".to_owned(),
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
