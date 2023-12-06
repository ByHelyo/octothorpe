use std::error::Error;

pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    Test { source: E },
}
