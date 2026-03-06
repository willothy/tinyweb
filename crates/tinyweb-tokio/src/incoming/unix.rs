use crate::io_compat::TokioIoCompat;

pub struct UnixIncoming(pub tokio::net::UnixListener);

impl tinyweb_core::incoming::Incoming for UnixIncoming {
    type Io = TokioIoCompat<tokio::net::UnixStream>;
    type Addr = tokio::net::unix::SocketAddr;
    type Error = std::io::Error;

    fn accept(
        &mut self,
    ) -> tinyweb_core::maybe_send::BoxFuture<'_, Result<(Self::Io, Self::Addr), Self::Error>> {
        Box::pin(async move {
            let (stream, addr) = self.0.accept().await?;
            Ok((TokioIoCompat::new(stream), addr))
        })
    }
}
