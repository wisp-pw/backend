use crate::prelude::*;

use std::net::SocketAddr;

pub struct WispSettings {
    pub host: SocketAddr,
    pub db_uri: String,
}

impl WispSettings {
    pub fn new() -> Result<WispSettings> {
        let host = dotenv!("HOST").to_string().parse::<SocketAddr>()?;
        Ok(WispSettings {
            host,
            db_uri: dotenv!("DB_URI").to_string(),
        })
    }
}
