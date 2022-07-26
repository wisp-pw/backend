mod prelude;
mod repositories;
mod response;
mod routes;
mod services;
mod settings;
mod state;
mod models;

use std::{net::SocketAddr, sync::Arc};

use axum::{routing::{get, post}, Router};
use prelude::*;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{settings::WispSettings, state::WispState};

pub async fn setup_app() -> Result<(Router, SocketAddr)> {
    // setup settings and state
    let settings = Arc::new(WispSettings::new()?);
    let state = WispState::new(&settings).await?;

    // setup setup_app
    let state = Arc::new(state);

    let bind_addr = settings.host.clone();
    let setup_app = Router::new()
        .route("/", get(routes::index::get))
        .route("/auth/register", post(routes::auth::register::post))
        .layer(Extension(settings))
        .layer(Extension(state));

    Ok((setup_app, bind_addr))
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

    // get setup_app instance
    let (setup_app, bind_addr) = setup_app().await?;

    // serve app
    info!("Listening on http://{bind_addr}");

    axum::Server::bind(&bind_addr)
        .serve(setup_app.into_make_service())
        .await?;

    Ok(())
}
