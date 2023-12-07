use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RestError {}

impl Display for RestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for RestError {}
