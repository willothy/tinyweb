use compio::io::compat::AsyncStream;

/// [`Incoming`](tinyweb_core::incoming::Incoming) implementation for compio Unix socket listeners.
pub struct UnixIncoming(pub compio::net::UnixListener);

impl tinyweb_core::incoming::Incoming for UnixIncoming {
    type Io = AsyncStream<compio::net::UnixStream>;
    type Addr = socket2::SockAddr;
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
