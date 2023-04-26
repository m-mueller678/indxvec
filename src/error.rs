use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

/// Shorthand type for custom error with payload String message
pub type IE = IdxError<String>;

#[derive(Debug)]
/// custom error
pub enum IdxError<T> {
    /// Non positive data dimension
    Size(T),
    /// Other error converted to IdxError
    Other(T),
}

impl<T> Error for IdxError<T> where T: Sized + Debug + Display {}

impl<T> Display for IdxError<T>
where
    T: Sized + Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdxError::Size(s) => write!(f, "Size of data must be positive: {s}"), 
            IdxError::Other(s) => write!(f, "Converted from: {s}"),
        }
    }
}

/// Convenience function for building IdxError<String>  
/// from error kind name and payload message, which can be either &str or String
pub fn ierror(kind: &str, msg: impl Into<String>) -> IE {
    match kind {
        "size" => IdxError::Size(msg.into()), 
        "other" => IdxError::Other(msg.into()),
        _ => IdxError::Other("Wrong error kind given to ierror".into())
    }
}
