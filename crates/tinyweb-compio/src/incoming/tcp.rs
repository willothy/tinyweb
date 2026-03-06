use compio::io::compat::AsyncStream;

pub struct TcpIncoming(pub compio::net::TcpListener);

impl tinyweb_core::incoming::Incoming for TcpIncoming {
    type Io = AsyncStream<compio::net::TcpStream>;
    type Addr = std::net::SocketAddr;
    type Error = std::io::Error;

    fn accept(
        &mut self,
    ) -> tinyweb_core::maybe_send::BoxFuture<'_, Result<(Self::Io, Self::Addr), Self::Error>> {
        Box::pin(async move {
            let (stream, addr) = self.0.accept().await?;
            Ok((AsyncStream::new(stream), addr))
        })
    }
}
