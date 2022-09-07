use crate::prelude::*;

use std::net::SocketAddr;

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum StorageType { Memory, Fs }
}

#[derive(Debug, Default)]
pub struct EmailSettings {
    pub enabled: bool,
    pub from: String,
    pub host: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug)]
pub struct WispSettings {
    pub host: SocketAddr,
    pub db_uri: String,
    pub email: EmailSettings,
    pub storage_type: StorageType,
    pub fs_storage_path: String,
    pub jwt_secret: String,
}

impl WispSettings {
    pub fn from_env() -> Result<WispSettings> {
        let host = env("HOST").parse::<SocketAddr>()?;

        let email_settings = EmailSettings {
            enabled: env("EMAIL_ENABLED").parse::<bool>().unwrap(),
            from: env("EMAIL_FROM"),
            host: env("EMAIL_HOST"),
            user: env("EMAIL_USER"),
            pass: env("EMAIL_PASS"),
        };

        Ok(WispSettings {
            host,
            db_uri: env("DB_URI"),
            email: email_settings,
            storage_type: env("STORAGE_TYPE").parse().unwrap(),
            fs_storage_path: env("FS_STORAGE_PATH"),
            jwt_secret: env("JWT_SECRET"),
        })
    }

    #[cfg(test)]
    pub fn for_test() -> WispSettings {
        WispSettings {
            host: SocketAddr::V4("127.0.0.1:0".parse().unwrap()),
            db_uri: "sqlite::memory:".to_string(),
            jwt_secret: "uwuuwuuwuuwu".to_string(),
            email: Default::default(),
            storage_type: StorageType::Memory,
            fs_storage_path: "".to_string(),
        }
    }
}
