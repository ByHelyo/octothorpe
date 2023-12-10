use std::{
    any,
    error::Error,
    fmt::{self, Display, Formatter},
};

use bytes::Bytes;
use http::StatusCode;
use serde_json::Value;
use url::ParseError;

use crate::rest_error::RestError;

#[derive(Debug)]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    Client {
        source: E,
    },
    UrlParse {
        source: ParseError,
    },
    DataType {
        source: serde_json::Error,
        typename: &'static str,
    },
    SlackService {
        status: StatusCode,
        data: Vec<u8>,
    },
    Slack {
        source: Value,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        Self::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }

    pub(crate) fn from_slack(val: Value) -> Self {
        Self::Slack { source: val }
    }

    pub(crate) fn server_error(status: StatusCode, bytes: &Bytes) -> Self {
        Self::SlackService {
            status,
            data: bytes.into_iter().copied().collect(),
        }
    }
}

impl<E> Display for ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Client { source } => write!(f, "client error: {}", source),
            Self::DataType { source, typename } => write!(
                f,
                "could not parse {} data from json: {}",
                typename, source
            ),
            Self::SlackService { status, .. } => {
                write!(f, "slack internal server error: {}", status)
            }
            Self::Slack { source } => {
                write!(f, "slack server error: {:?}", source)
            }
            Self::UrlParse { source } => {
                write!(f, "failed to parse url: {}", source)
            }
        }
    }
}

impl<E> Error for ApiError<E> where E: Error + Send + Sync + 'static {}

impl<E> From<ParseError> for ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    fn from(value: ParseError) -> Self {
        Self::UrlParse { source: value }
    }
}

impl From<RestError> for ApiError<RestError> {
    fn from(value: RestError) -> Self {
        Self::Client { source: value }
    }
}
