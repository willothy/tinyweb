use std::sync::Arc;

use crate::{body::Body, maybe_send::BoxFuture, service::Service};

#[cfg(feature = "send")]
pub(crate) trait ErasedService: Send + Sync + 'static {
    fn call_erased(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>>;
}

#[cfg(not(feature = "send"))]
pub(crate) trait ErasedService: 'static {
    fn call_erased(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>>;
}

#[derive(Clone)]
pub struct Route {
    inner: Arc<dyn ErasedService>,
}

impl Route {
    pub(crate) fn new<S: Service>(service: S) -> Self {
        Self {
            inner: Arc::new(Erased(service)),
        }
    }

    pub(crate) fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        self.inner.call_erased(req)
    }
}

impl Service for Route {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        self.inner.call_erased(req)
    }
}

struct Erased<S>(S);

impl<S: Service> ErasedService for Erased<S> {
    fn call_erased(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        self.0.call(req)
    }
}
