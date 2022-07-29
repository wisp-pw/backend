use crate::prelude::*;

use std::net::SocketAddr;

pub struct WispSettings {
    pub host: SocketAddr,
    pub db_uri: String,
}

impl WispSettings {
    pub fn from_env() -> Result<WispSettings> {
        let host = dotenv!("HOST").to_string().parse::<SocketAddr>()?;
        Ok(WispSettings {
            host,
            db_uri: dotenv!("DB_URI").to_string(),
        })
    }

    #[cfg(test)]
    pub fn for_test() -> WispSettings {
        WispSettings {
            host: "127.0.0.1:3000".to_string().parse::<SocketAddr>().unwrap(),
            db_uri: "sqlite::memory:".to_string(),
        }
    }
}
