use crate::maybe_send::{BoxFuture, MaybeSend, MaybeSync};

/// Runtime abstraction for spawning futures.
pub trait Runtime: Clone + MaybeSend + MaybeSync + 'static {
    /// Spawn a future onto the runtime.
    fn spawn(&self, fut: BoxFuture<'static, ()>);
}
