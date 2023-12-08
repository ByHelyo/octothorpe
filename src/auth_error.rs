use std::{error::Error, fmt::Display};

use http::header::InvalidHeaderValue;

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    HeaderValue { source: InvalidHeaderValue },
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HeaderValue { source } => {
                write!(f, "header value error: {}", source)
            }
        }
    }
}

impl Error for AuthError {}

impl From<InvalidHeaderValue> for AuthError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::HeaderValue { source: value }
    }
}
