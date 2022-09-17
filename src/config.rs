use color_eyre::eyre::eyre;
use strum::EnumString;

use crate::prelude::*;

use std::{fs, net::SocketAddr, path::PathBuf};

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum StorageType {
    Memory,
    Fs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub storage_type: StorageType,
    pub fs_storage_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfig {
    pub enabled: bool,
    pub from: String,
    pub host: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTConfig {
    pub issuer: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WispConfig {
    pub host: SocketAddr,
    pub db_uri: String,

    pub file: FileConfig,
    pub email: EmailConfig,
    pub jwt: JWTConfig,
}

impl WispConfig {
    pub fn from_file() -> Result<WispConfig> {
        let config_file_path = PathBuf::from("config.toml");
        if config_file_path.exists() {
            let config_file_content = fs::read_to_string(config_file_path)?;
            let config = toml::from_str::<WispConfig>(&config_file_content)?;

            Ok(config)
        } else {
            let default_config = WispConfig {
                host: "127.0.0.1:3000".to_string().parse::<SocketAddr>().unwrap(),
                db_uri: "sqlite::memory:".to_string(),

                email: EmailConfig {
                    enabled: false,
                    from: "".to_string(),
                    host: "".to_string(),
                    user: "".to_string(),
                    pass: "".to_string(),
                },

                file: FileConfig {
                    storage_type: StorageType::Memory,
                    fs_storage_path: "".to_string(),
                },

                jwt: JWTConfig {
                    issuer: "wisp".to_string(),
                    secret: "wisp".to_string(),
                },
            };

            let default_config_str = toml::to_string(&default_config)?;
            fs::write(config_file_path, default_config_str)?;

            Err(eyre!(
                "No config file found! Generated a default one for you."
            ))
        }
    }

    #[cfg(test)]
    pub fn for_test() -> WispConfig {
        WispConfig {
            host: "127.0.0.1:3000".to_string().parse::<SocketAddr>().unwrap(),
            db_uri: "sqlite::memory:".to_string(),

            email: EmailConfig {
                enabled: false,
                from: "".to_string(),
                host: "".to_string(),
                user: "".to_string(),
                pass: "".to_string(),
            },

            file: FileConfig {
                storage_type: StorageType::Memory,
                fs_storage_path: "".to_string(),
            },

            jwt: JWTConfig {
                issuer: "wisp".to_string(),
                secret: "wisp".to_string(),
            },
        }
    }
}
