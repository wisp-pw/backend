pub use color_eyre::Result;
pub use dotenv_codegen::dotenv;
pub use std::sync::Arc;
pub use tracing::log::{debug, error, info, trace, warn};

pub use crate::models::user::UserDTO;
pub use crate::option::OptionExtension;

pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
pub use serde::{Deserialize, Serialize};
pub use serde_json::json;
pub use email_address::EmailAddress;

pub use crate::response::*;
pub use crate::settings::*;
pub use crate::state::*;

// testing imports

#[cfg(test)]
pub use tower::{Service, ServiceExt};

#[cfg(test)]
pub use crate::test::*;