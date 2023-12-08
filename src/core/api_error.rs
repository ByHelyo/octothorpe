use std::{any, error::Error, fmt::Display};

#[derive(Debug)]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    Test {
        source: E,
    },
    DataType {
        source: serde_json::Error,
        typename: &'static str,
    },
    SlackService {
        status: http::StatusCode,
        data: Vec<u8>,
    },
    Slack {
        source: serde_json::Value,
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

    pub(crate) fn from_slack(val: serde_json::Value) -> Self {
        Self::Slack { source: val }
    }
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    pub(crate) fn server_error(
        status: http::StatusCode,
        bytes: &bytes::Bytes,
    ) -> Self {
        Self::SlackService {
            status: status,
            data: bytes.into_iter().copied().collect(),
        }
    }
}

impl<E> Display for ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::Test { .. } => write!(f, "remove"),
        }
    }
}

impl<E> Error for ApiError<E> where E: Error + Send + Sync + 'static {}
