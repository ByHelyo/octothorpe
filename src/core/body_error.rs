use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum BodyError {}

impl Display for BodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for BodyError {}
