use crate::prelude::*;

use std::net::SocketAddr;

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum StorageType { Memory, Fs }
}

pub struct WispSettings {
    pub host: SocketAddr,
    pub db_uri: String,
    pub email_enabled: bool,
    pub email_from: String,
    pub email_host: String,
    pub email_user: String,
    pub email_pass: String,
    pub storage_type: StorageType,
    pub fs_storage_path: String,
}

impl WispSettings {
    pub fn from_env() -> Result<WispSettings> {
        let host = env("HOST").parse::<SocketAddr>()?;
        Ok(WispSettings {
            host,
            db_uri: env("DB_URI"),
            email_enabled: env("EMAIL_ENABLED").parse::<bool>().unwrap(),
            email_from: env("EMAIL_FROM"),
            email_host: env("EMAIL_HOST"),
            email_user: env("EMAIL_USER"),
            email_pass: env("EMAIL_PASS"),
            storage_type: env("STORAGE_TYPE").to_string().parse().unwrap(),
            fs_storage_path: env("FS_STORAGE_PATH").to_string(),
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
            email_user: "".to_string(),
            email_pass: "".to_string(),
            storage_type: StorageType::Memory,
            fs_storage_path: "".to_string(),
        }
    }
}
