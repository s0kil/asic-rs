use serde_json::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum RPCError {
    StatusCheckFailed(String),
    DeserializationFailed(Error),
    ConnectionFailed,
}

impl Display for RPCError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RPCError::StatusCheckFailed(message) => {
                write!(f, "Command returned with error status: {}", message)
            }
            RPCError::DeserializationFailed(error) => {
                write!(f, "Failed to deserialize result: {}", error)
            }
            RPCError::ConnectionFailed => {
                write!(f, "Failed to connect to RPC API")
            }
        }
    }
}

impl std::error::Error for RPCError {}

impl From<Error> for RPCError {
    fn from(value: Error) -> Self {
        Self::DeserializationFailed(value)
    }
}
