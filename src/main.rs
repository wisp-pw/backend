mod error;
mod models;
mod option;
mod prelude;
mod repositories;
mod response;
mod routes;
mod services;
mod settings;
mod state;
mod test;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};
use prelude::*;
use repositories::file::{FileRepository, FsFileRepository, MemoryFileRepository};
use services::file_save::FileSaveService;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{settings::WispSettings, state::WispState};

pub async fn setup_app(settings: Arc<WispSettings>) -> Result<(Router, SocketAddr)> {
    // setup state
    let state = WispState::new(&settings).await?;
    let state = Arc::new(state);

    let file_repository: Box<dyn FileRepository + Send + Sync> = match settings.storage_type {
        StorageType::Memory => Box::new(MemoryFileRepository::new()),
        StorageType::Fs => Box::new(FsFileRepository::new(&settings)),
    };

    let file_save_service = FileSaveService::new(file_repository);

    let bind_addr = settings.host;
    let setup_app = Router::new()
        .route("/", get(routes::index::get))
        .route("/auth/register", post(routes::auth::register::post))
        .route("/auth/login", post(routes::auth::login::post))
        .route("/upload", post(routes::upload::post))
        .layer(Extension(settings))
        .layer(Extension(state))
        .layer(Extension(file_save_service));

    Ok((setup_app, bind_addr))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    // setup logging
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .pretty()
        .finish()
        .init();

    // setup settings
    let settings = WispSettings::from_env()?;
    let settings = Arc::new(settings);

    // get router and bind addr
    let (setup_app, bind_addr) = setup_app(settings).await?;

    // serve app
    info!("Listening on http://{bind_addr}");

    axum::Server::bind(&bind_addr)
        .serve(setup_app.into_make_service())
        .await?;

    Ok(())
}
