use core::pin::Pin;

#[derive(Clone, Default)]
pub struct TokioRuntime;

impl tinyweb_core::runtime::Runtime for TokioRuntime {
    fn spawn(
        &self,
        fut: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ) {
        tokio::spawn(fut);
    }
}
