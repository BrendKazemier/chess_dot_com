#![allow(dead_code)]

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("http error")]
    Http(#[from] reqwest::Error),

    #[error("serialization error")]
    Json(#[from] serde_json::Error),
    
    #[error("API returned error: {0:?}")]
    Api(ApiErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub code: u16,
    pub message: String,
}