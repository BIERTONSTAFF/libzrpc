use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::middleware::error::MiddlewareError;

#[macro_export]
macro_rules! proc_err {
    ($kind:ident) => {
        Err(ProcedureError::$kind)
    };
}

#[macro_export]
macro_rules! proc_ok {
    ($v:expr) => {{
        use libzrpc::types::dt::ZRpcDtAuto;

        Ok($v.to_zdt())
    }};
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcedureError {
    NotFound,
    InvalidParameters,
    Internal,
    Middleware(String),
}

impl Display for ProcedureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcedureError::NotFound => write!(f, "NotFound"),
            ProcedureError::InvalidParameters => write!(f, "InvalidParameters"),
            ProcedureError::Internal => write!(f, "Internal"),
            ProcedureError::Middleware(e) => write!(f, "Middleware(\"{}\")", e),
        }
    }
}

impl From<MiddlewareError> for ProcedureError {
    fn from(err: MiddlewareError) -> Self {
        ProcedureError::Middleware(err.0)
    }
}
