use core::pin::Pin;

use crate::io_compat::TokioIoCompat;

pub struct TcpIncoming(pub tokio::net::TcpListener);

impl tinyweb_core::incoming::Incoming for TcpIncoming {
    type Io = TokioIoCompat<tokio::net::TcpStream>;
    type Addr = std::net::SocketAddr;
    type Error = std::io::Error;

    fn accept(
        &mut self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(Self::Io, Self::Addr), Self::Error>> + Send + '_,
        >,
    > {
        Box::pin(async move {
            let (stream, addr) = self.0.accept().await?;
            Ok((TokioIoCompat::new(stream), addr))
        })
    }
}
