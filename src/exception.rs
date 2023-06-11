use std::{error::Error, fmt};

#[derive(Debug)]
pub struct NetworkRexError {
    pub message: String,
}

impl Error for NetworkRexError {}

impl fmt::Display for NetworkRexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
