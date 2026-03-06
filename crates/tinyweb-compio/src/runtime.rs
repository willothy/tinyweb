#[derive(Clone, Default)]
pub struct CompioRuntime;

impl tinyweb_core::runtime::Runtime for CompioRuntime {
    fn spawn(&self, fut: tinyweb_core::maybe_send::BoxFuture<'static, ()>) {
        compio::runtime::spawn(fut).detach();
    }
}
