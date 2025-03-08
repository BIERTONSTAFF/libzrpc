use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::{error::ZRpcError, log};

pub struct TcpTransport {
    stream: TcpStream,
}

impl TcpTransport {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn send(&mut self, bytes: &[u8]) -> Result<(), ZRpcError> {
        let len = (bytes.len() as u32).to_be_bytes();
        self.stream.write_all(&len).await.map_err(ZRpcError::Io)?;
        self.stream.write_all(bytes).await.map_err(ZRpcError::Io)?;

        log!(
            "[TcpTransport:{:?}] {} bytes were sent",
            std::thread::current().id(),
            len.len() + bytes.len()
        );

        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>, ZRpcError> {
        let mut len = [0u8; 4];
        self.stream
            .read_exact(&mut len)
            .await
            .map_err(ZRpcError::Io)?;

        let mut buf = vec![0u8; u32::from_be_bytes(len) as usize];
        self.stream
            .read_exact(&mut buf)
            .await
            .map_err(ZRpcError::Io)?;

        log!(
            "[TcpTransport:{:?}] {} bytes were received",
            std::thread::current().id(),
            len.len() + buf.len()
        );

        Ok(buf)
    }
}
