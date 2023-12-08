use std::{error::Error, fmt::Display};

use crate::auth_error::AuthError;

#[derive(Debug)]
pub enum RestError {
    AuthError { source: AuthError },
    Http { source: http::Error },
    Communication { source: reqwest::Error },
}

impl Display for RestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestError::AuthError { source } => {
                write!(f, "eror setting auth header: {}", source)
            }
            RestError::Http { source } => write!(f, "http error: {}", source),
            RestError::Communication { source } => {
                write!(f, "communication with slack: {}", source)
            }
        }
    }
}

impl Error for RestError {}

impl From<AuthError> for RestError {
    fn from(value: AuthError) -> Self {
        Self::AuthError { source: value }
    }
}

impl From<http::Error> for RestError {
    fn from(value: http::Error) -> Self {
        Self::Http { source: value }
    }
}

impl From<reqwest::Error> for RestError {
    fn from(value: reqwest::Error) -> Self {
        Self::Communication { source: value }
    }
}
