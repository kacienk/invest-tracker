use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct UuidParseError {
    description: String,
}

impl UuidParseError {
    pub fn new(description: &str) -> Self {
        UuidParseError {
            description: description.to_string(),
        }
    }
}

impl fmt::Display for UuidParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl StdError for UuidParseError {}
