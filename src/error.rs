use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum LinkedinError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("Challenge encountered: {0}")]
    Challenge(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Request failed: {0}")]
    InvalidURN(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("Header value error: {0}")]
    Header(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Header to string error: {0}")]
    HeaderToStr(#[from] reqwest::header::ToStrError),
}
