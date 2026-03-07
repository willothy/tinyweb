use crate::{
    body::Body,
    layer::Layer,
    maybe_send::{BoxFuture, MaybeSend, MaybeSync},
};

pub trait Service: Clone + MaybeSend + MaybeSync + 'static {
    fn call(&self, req: http::Request<h2::RecvStream>) -> BoxFuture<'static, http::Response<Body>>;

    fn layer<L: Layer<Self>>(self, layer: L) -> L::Service
    where
        Self: Sized,
    {
        layer.layer(self)
    }
}

pub trait IntoService<T> {
    type Service: Service;
    fn into_service(self) -> Self::Service;
}

impl<S: Service> IntoService<()> for S {
    type Service = S;
    fn into_service(self) -> Self::Service {
        self
    }
}
