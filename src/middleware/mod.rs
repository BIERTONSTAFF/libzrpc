use error::MiddlewareError;

use crate::types::req::ZRpcReq;

pub mod error;

pub trait Middleware: Send + Sync {
    fn before_call(&self, req: &ZRpcReq) -> Result<(), MiddlewareError>;
}
