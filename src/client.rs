use std::{net::Ipv4Addr, time::Duration};

use tokio::{net::TcpStream, time::timeout};

use crate::{
    error::ZRpcError,
    transport::tcp::TcpTransport,
    types::{dt::ZRpcDt, proc_error::ProcedureError, req::ZRpcReq},
};

pub struct ZRpcClient {
    transport: TcpTransport,
    timeout: Duration,
}

impl ZRpcClient {
    pub async fn new(addr: (Ipv4Addr, u16)) -> Result<Self, ZRpcError> {
        let stream = TcpStream::connect(addr).await.map_err(ZRpcError::Io)?;
        let transport = TcpTransport::new(stream);

        Ok(Self {
            transport,
            timeout: Duration::from_secs(30),
        })
    }

    pub fn set_timeout(&mut self, duration: Duration) {
        self.timeout = duration;
    }

    pub async fn call(&mut self, proc: &str, params: Vec<ZRpcDt>) -> Result<ZRpcDt, ZRpcError> {
        self.execute(ZRpcReq::new(proc, params)).await
    }

    async fn execute(&mut self, req: ZRpcReq) -> Result<ZRpcDt, ZRpcError> {
        let bytes =
            bincode::serialize(&req).map_err(|e| ZRpcError::Serialization(e.to_string()))?;

        timeout(self.timeout, async {
            self.transport.send(&bytes).await?;

            let bytes = self.transport.receive().await?;
            let res: Result<ZRpcDt, ProcedureError> = bincode::deserialize(&bytes)
                .map_err(|e| ZRpcError::Serialization(e.to_string()))?;

            res.map_err(ZRpcError::Procedure)
        })
        .await
        .map_err(|_| ZRpcError::TimeoutError)?
    }
}
