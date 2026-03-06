use crate::maybe_send::{BoxFuture, MaybeSend, MaybeSync};

pub trait Runtime: Clone + MaybeSend + MaybeSync + 'static {
    fn spawn(&self, fut: BoxFuture<'static, ()>);
}
