/// [`Runtime`](tinyweb_core::runtime::Runtime) implementation backed by tokio.
#[derive(Clone, Default)]
pub struct TokioRuntime;

impl tinyweb_core::runtime::Runtime for TokioRuntime {
    fn spawn(&self, fut: tinyweb_core::maybe_send::BoxFuture<'static, ()>) {
        tokio::spawn(fut);
    }
}
