use futures::io::{AsyncRead, AsyncWrite};

pub trait Transport: AsyncRead + AsyncWrite + Send + 'static {}

impl<T> Transport for T where T: AsyncRead + AsyncWrite + Send + 'static {}

pub trait Runtime: Send + Sync + 'static {
    fn spawn(&self, task: impl Future<Output = ()> + Send + 'static);
}

pub struct TokioRuntime;

impl Runtime for TokioRuntime {
    fn spawn(&self, task: impl Future<Output = ()> + Send + 'static) {
        tokio::spawn(task);
    }
}
