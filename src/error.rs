use std::error::{Error as StdError};

#[derive(Debug)]
pub struct Error(pub Box<dyn StdError + Send + Sync>);
impl<I: Into<Box<dyn StdError + Send + Sync>>> From<I> for Error {
    fn from(e: I) -> Self {
        Error(e.into())
    }
}
