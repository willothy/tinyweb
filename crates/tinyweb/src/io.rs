use std::task::{Context, Poll, ready};

pin_project_lite::pin_project! {
    pub(crate) struct TokioIo<T> {
        #[pin]
        inner: T,
    }
}

impl<T> TokioIo<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: futures_io::AsyncRead> tokio::io::AsyncRead for TokioIo<T> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let slice = buf.initialize_unfilled();
        let n = ready!(self.project().inner.poll_read(cx, slice))?;
        buf.advance(n);
        Poll::Ready(Ok(()))
    }
}

impl<T: futures_io::AsyncWrite> tokio::io::AsyncWrite for TokioIo<T> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.project().inner.poll_close(cx)
    }
}
