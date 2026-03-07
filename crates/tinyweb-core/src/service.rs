use crate::{
    body::Body,
    layer::Layer,
    maybe_send::{BoxFuture, MaybeSend, MaybeSync},
};

/// A cloneable request handler that takes a request and returns a response.
pub trait Service: Clone + MaybeSend + MaybeSync + 'static {
    /// Handle the request and return a response.
    fn call(&self, req: http::Request<h2::RecvStream>) -> BoxFuture<'static, http::Response<Body>>;

    /// Wrap this service with a middleware layer.
    fn layer<L: Layer<Self>>(self, layer: L) -> L::Service
    where
        Self: Sized,
    {
        layer.layer(self)
    }
}

/// Convert a type into a [`Service`].
pub trait IntoService<T> {
    /// The resulting service type.
    type Service: Service;
    /// Perform the conversion.
    fn into_service(self) -> Self::Service;
}

impl<S: Service> IntoService<()> for S {
    type Service = S;
    fn into_service(self) -> Self::Service {
        self
    }
}
