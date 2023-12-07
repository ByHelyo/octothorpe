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
            Self::Test { .. } => write!(f, "remove"),
        }
    }
}

impl<E> Error for ApiError<E> where E: Error + Send + Sync + 'static {}
