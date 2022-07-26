mod prelude;
mod repositories;
mod response;
mod routes;
mod services;
mod settings;
mod state;

use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use prelude::*;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{settings::WispSettings, state::WispState};

pub async fn router() -> Result<(Router, SocketAddr)> {
    // setup settings and state
    let settings = Arc::new(WispSettings::new()?);
    let state = WispState::new(&settings).await?;

    // setup router
    let state = Arc::new(state);

    let bind_addr = settings.host.clone();
    let router = Router::new()
        .route("/", get(routes::index::get))
        .layer(Extension(settings))
        .layer(Extension(state));

    Ok((router, bind_addr))
}

#[tokio::main]
async fn main() -> Result<()> {
    // setup logging
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .pretty()
        .finish()
        .init();

    // get router instance
    let (router, bind_addr) = router().await?;

    // serve app
    info!("Listening on http://{bind_addr}");

    axum::Server::bind(&bind_addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
