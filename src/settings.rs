use crate::prelude::*;

use std::net::SocketAddr;

pub struct WispSettings {
    pub host: SocketAddr,
    pub db_uri: String,
    pub email_enabled: bool,
    pub email_from: String,
    pub email_host: String,
    pub email_port: u16,
    pub email_user: String,
    pub email_pass: String,
}

impl WispSettings {
    pub fn from_env() -> Result<WispSettings> {
        let host = dotenv!("HOST").to_string().parse::<SocketAddr>()?;
        Ok(WispSettings {
            host,
            db_uri: dotenv!("DB_URI").to_string(),
            email_enabled: dotenv!("EMAIL_ENABLED")
                .to_string()
                .parse::<bool>()
                .unwrap(),
            email_from: dotenv!("EMAIL_FROM").to_string(),
            email_host: dotenv!("EMAIL_HOST").to_string(),
            email_port: dotenv!("EMAIL_PORT").to_string().parse::<u16>().unwrap(),
            email_user: dotenv!("EMAIL_USER").to_string(),
            email_pass: dotenv!("EMAIL_PASS").to_string(),
        })
    }

    #[cfg(test)]
    pub fn for_test() -> WispSettings {
        WispSettings {
            host: "127.0.0.1:3000".to_string().parse::<SocketAddr>().unwrap(),
            db_uri: "sqlite::memory:".to_string(),
            email_enabled: false,
            email_from: "".to_string(),
            email_host: "".to_string(),
            email_port: 0,
            email_user: "".to_string(),
            email_pass: "".to_string(),
        }
    }
}
