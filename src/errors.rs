//! Possible error types.

use serde::Deserialize;
use thiserror::Error;

/// An error caused by sending a request to BrawlStars.
#[derive(Debug, Error)]
pub enum RequestError {
    // #[error("Test")]
    // RandomError(#[source] std::sync::Arc<std::sync::Mutex<Box<dyn std::error::Error>>>),

    // #[error("Test")]
    // Ph(std::marker::PhantomData<R>),
    /// A BrawlStars API error.
    #[error("A BrawlStars's error: {0}")]
    Api(#[source] ApiError),

    #[error("An error while parsing JSON: {source} (raw: {raw:?})")]
    InvalidJson {
        #[source]
        source: serde_json::Error,
        /// The raw string JSON that couldn't been parsed
        raw: Box<str>,
    },

    /// Network error while sending a request.
    #[error("A network error: {0}")]
    // NOTE: this variant must not be created by anything except the From impl
    Network(#[source] reqwest::Error),
}

/// A kind of an API error.
#[derive(Debug, Error, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct ApiError {
    reason: String,
    message: Option<String>,
    r#type: Option<String>,
    detail: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        RequestError::Network(hide_token(error))
    }
}

// impl From<&dyn std::error::Error> for RequestError {
//     fn from(_: &dyn std::error::Error) -> Self {
//         RequestError::Api(ApiError)
//     }
// }

pub(crate) fn hide_token(error: reqwest::Error) -> reqwest::Error {
    // NOTE: probably good idea to hide any tokens
    error
}
