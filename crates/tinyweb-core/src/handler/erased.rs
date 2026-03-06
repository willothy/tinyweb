use std::{pin::Pin, sync::Arc};

use crate::{body::Body, handler::Handler};

pub(crate) trait ErasedHandler: Send + Sync + 'static {
    fn call_erased(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> Pin<Box<dyn Future<Output = http::Response<Body>> + Send>>;
}

pub(crate) fn erase_handler<H, T>(handler: H) -> Arc<dyn ErasedHandler>
where
    T: 'static,
    H: Handler<T>,
{
    Arc::new(Erase::new(handler))
}

struct Erase<H, T> {
    handler: H,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<H, T> Erase<H, T> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            marker: std::marker::PhantomData,
        }
    }
}

impl<H, T> ErasedHandler for Erase<H, T>
where
    T: 'static,
    H: Handler<T>,
{
    fn call_erased(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> Pin<Box<dyn Future<Output = http::Response<Body>> + Send>> {
        self.handler.call(req)
    }
}
