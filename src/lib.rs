pub mod api;
mod auth;
mod auth_error;
pub mod core;
mod rest_error;
mod slack;

pub use self::slack::Slack;

#[cfg(test)]
mod test;
