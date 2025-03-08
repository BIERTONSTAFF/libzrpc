use std::net::Ipv4Addr;

use libzrpc::{
    add_procs,
    middleware::{error::MiddlewareError, Middleware},
    middleware_err, proc_err, proc_ok,
    server::ZRpcServer,
    types::{dt::ZRpcDt, proc_error::ProcedureError},
};

pub struct AuthMiddleware {
    api_key: String,
}

impl Middleware for AuthMiddleware {
    fn before_call(&self, req: &libzrpc::types::req::ZRpcReq) -> Result<(), MiddlewareError> {
        if let Some(ZRpcDt::String(key)) = req.1.first() {
            if key == &self.api_key {
                Ok(())
            } else {
                middleware_err!("Unauthorized")
            }
        } else {
            middleware_err!("Missing apiKey")
        }
    }
}

fn add(p: &Vec<ZRpcDt>) -> Result<ZRpcDt, ProcedureError> {
    match (&p[1], &p[2]) {
        (ZRpcDt::Int32(a), ZRpcDt::Int32(b)) => proc_ok!(a + b),
        _ => proc_err!(InvalidParameters),
    }
}

#[tokio::main]
async fn main() {
    let mut server = ZRpcServer::new((Ipv4Addr::LOCALHOST, 3000)).await.unwrap();

    server
        .add_middleware(AuthMiddleware {
            api_key: "SECRET_KEY".to_string(),
        })
        .await;

    add_procs!(server, add);

    server.start().await.unwrap()
}
