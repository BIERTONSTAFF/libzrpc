use std::{collections::HashMap, net::Ipv4Addr, sync::Arc};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{
    error::ZRpcError,
    log,
    middleware::Middleware,
    transport::tcp::TcpTransport,
    types::{dt::ZRpcDt, proc_error::ProcedureError, req::ZRpcReq},
};

#[macro_export]
macro_rules! add_procs {
    ($server:expr, $($proc:ident),*) => {
        $(
            $server.add_proc(stringify!($proc), |params| {
                $proc(params)
            }).await;
        )*
    };
}

pub struct ZRpcServer {
    listener: TcpListener,
    procs: Arc<
        Mutex<
            HashMap<
                String,
                Box<dyn Fn(&Vec<ZRpcDt>) -> Result<ZRpcDt, ProcedureError> + Send + Sync>,
            >,
        >,
    >,
    middleware: Arc<Mutex<Vec<Box<dyn Middleware>>>>,
}

impl ZRpcServer {
    pub async fn new(addr: (Ipv4Addr, u16)) -> Result<Self, ZRpcError> {
        let listener = TcpListener::bind(addr).await.map_err(ZRpcError::Io)?;

        Ok(Self {
            listener,
            procs: Arc::new(Mutex::new(HashMap::new())),
            middleware: Arc::new(Mutex::new(vec![])),
        })
    }

    pub async fn start(&mut self) -> Result<(), ZRpcError> {
        while let Ok((stream, _)) = self.listener.accept().await {
            let procs = self.procs.clone();
            let middleware = self.middleware.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_stream(stream, &procs, &middleware).await {
                    eprintln!("Failed to handle stream: {}", e);
                }
            });
        }

        Ok(())
    }

    pub async fn add_proc<F>(&mut self, name: &str, proc: F)
    where
        F: Fn(&Vec<ZRpcDt>) -> Result<ZRpcDt, ProcedureError> + 'static + Send + Sync,
    {
        log!("[ZRpcServer] '{}' procedure has been loaded", name);

        self.procs
            .lock()
            .await
            .insert(name.to_string(), Box::new(proc));
    }

    pub async fn add_middleware<M: Middleware + 'static>(&mut self, middleware: M) {
        self.middleware.lock().await.push(Box::new(middleware));
    }

    async fn handle_stream(
        stream: TcpStream,
        procs: &Arc<
            Mutex<
                HashMap<
                    String,
                    Box<dyn Fn(&Vec<ZRpcDt>) -> Result<ZRpcDt, ProcedureError> + Send + Sync>,
                >,
            >,
        >,
        middleware: &Arc<Mutex<Vec<Box<dyn Middleware>>>>,
    ) -> Result<(), ZRpcError> {
        let mut transport = TcpTransport::new(stream);

        loop {
            let buf = match transport.receive().await {
                Ok(buf) => buf,
                Err(ZRpcError::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    log!(
                        "[ZRpcServer:{:?}] Connection closed",
                        std::thread::current().id()
                    );
                    return Ok(());
                }
                Err(e) => return Err(e),
            };

            let req: ZRpcReq =
                bincode::deserialize(&buf).map_err(|e| ZRpcError::Serialization(e.to_string()))?;

            let res = {
                let middleware_lock = middleware.lock().await;
                let res = middleware_lock
                    .iter()
                    .try_fold((), |_, m| m.before_call(&req));
                drop(middleware_lock);

                match res {
                    Ok(_) => match procs.lock().await.get(&req.0) {
                        Some(proc) => proc(&req.1),
                        None => Err(ProcedureError::NotFound),
                    },
                    Err(e) => Err(e.into()),
                }
            };

            let bytes =
                bincode::serialize(&res).map_err(|e| ZRpcError::Serialization(e.to_string()))?;

            transport.send(&bytes).await?;

            log!(
                "[ZRpcServer:{:?}] Response sent: {:?}",
                std::thread::current().id(),
                res
            );
        }
    }
}
