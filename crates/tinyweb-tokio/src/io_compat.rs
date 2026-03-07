use std::task::{Context, Poll, ready};

pin_project_lite::pin_project! {
    /// Wrapper that adapts a tokio `AsyncRead + AsyncWrite` type to `futures_io`.
    pub struct TokioIoCompat<T> {
        #[pin]
        inner: T,
    }
}

impl<T> TokioIoCompat<T> {
    /// Wrap a tokio IO type.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: tokio::io::AsyncRead> futures_io::AsyncRead for TokioIoCompat<T> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut read_buf = tokio::io::ReadBuf::new(buf);
        ready!(self.project().inner.poll_read(cx, &mut read_buf))?;
        Poll::Ready(Ok(read_buf.filled().len()))
    }
}

impl<T: tokio::io::AsyncWrite> futures_io::AsyncWrite for TokioIoCompat<T> {
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

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.project().inner.poll_shutdown(cx)
    }
}
