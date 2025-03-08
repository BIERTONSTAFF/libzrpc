use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::types::proc_error::ProcedureError;

#[derive(Debug)]
pub enum ZRpcError {
    Io(std::io::Error),
    Serialization(String),
    TimeoutError,
    Procedure(ProcedureError),
}

impl From<std::io::Error> for ZRpcError {
    fn from(err: std::io::Error) -> Self {
        ZRpcError::Io(err)
    }
}

impl Display for ZRpcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ZRpcError::Io(e) => write!(f, "IoError: {}", e),
            ZRpcError::Serialization(e) => write!(f, "SerializationError: {}", e),
            ZRpcError::TimeoutError => write!(f, "Timeout"),
            ZRpcError::Procedure(e) => write!(f, "ProcedureError: {}", e),
        }
    }
}

impl Error for ZRpcError {}
