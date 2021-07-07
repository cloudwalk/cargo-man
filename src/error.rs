use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    #[allow(dead_code)]
    pub fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.message)
    }
}
