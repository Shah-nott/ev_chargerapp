// utils/error.rs
//! Custom error definitions for the EV Charger application

use actix_web::ResponseError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "API request failed: {}", _0)]
    ApiRequestFailed(String),

    #[display(fmt = "Bluetooth error: {}", _0)]
    BluetoothError(String),

    #[display(fmt = "Unknown error")] 
    Unknown,
}

impl ResponseError for AppError {}
