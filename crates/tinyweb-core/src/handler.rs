mod erased;

use crate::{
    body::Body,
    layer::Layer,
    maybe_send::{BoxFuture, MaybeSend, MaybeSync},
    response::IntoResponse,
    service::{IntoService, Service},
};

pub use erased::Route;

/// Trait for async handler functions.
///
/// Implemented automatically for async functions with up to 10 arguments
/// via declarative macros.
pub trait Handler<T>: MaybeSend + MaybeSync + 'static {
    /// Call the handler with the given request.
    fn call(&self, req: http::Request<h2::RecvStream>) -> BoxFuture<'static, http::Response<Body>>;

    /// Wrap this handler with a middleware layer.
    fn layer<L>(self, layer: L) -> L::Service
    where
        Self: Sized + Clone,
        T: 'static,
        L: Layer<HandlerService<Self, T>>,
    {
        layer.layer(HandlerService::new(self))
    }
}

impl<H: Handler<T> + Clone, T: 'static> IntoService<(H, T)> for H {
    type Service = HandlerService<H, T>;
    fn into_service(self) -> Self::Service {
        HandlerService::new(self)
    }
}

/// Wraps a [`Handler`] into a [`Service`].
pub struct HandlerService<H, T> {
    handler: H,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<H: Clone, T> Clone for HandlerService<H, T> {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<H, T> HandlerService<H, T> {
    /// Create a new `HandlerService` from a handler.
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            marker: std::marker::PhantomData,
        }
    }
}

impl<H, T> Service for HandlerService<H, T>
where
    T: 'static,
    H: Handler<T> + Clone,
{
    fn call(&self, req: http::Request<h2::RecvStream>) -> BoxFuture<'static, http::Response<Body>> {
        self.handler.call(req)
    }
}

impl<F, Fut, Res> Handler<()> for F
where
    F: Fn() -> Fut + Clone + MaybeSend + MaybeSync + 'static,
    Fut: Future<Output = Res> + MaybeSend + 'static,
    Res: IntoResponse + 'static,
{
    fn call(
        &self,
        _req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        let f = self.clone();
        Box::pin(async move { f().await.into_response() })
    }
}

macro_rules! impl_handler {
    (($last_ty:ident, $last_var:ident)) => {
        impl<HandlerFn, Fut, Res, $last_ty> $crate::handler::Handler<($last_ty,)> for HandlerFn
        where
            HandlerFn: Fn($last_ty) -> Fut + Clone + $crate::maybe_send::MaybeSend + $crate::maybe_send::MaybeSync + 'static,
            Fut: core::future::Future<Output = Res> + $crate::maybe_send::MaybeSend + 'static,
            Res: $crate::response::IntoResponse + 'static,
            $last_ty: $crate::extract::FromRequest + $crate::maybe_send::MaybeSend + 'static,
        {
            fn call(
                &self,
                req: http::Request<h2::RecvStream>,
            ) -> $crate::maybe_send::BoxFuture<'static, http::Response<$crate::body::Body>> {
                let f = self.clone();

                Box::pin(async move {
                    let $last_var: $last_ty =
                        match <$last_ty as $crate::extract::FromRequest>::from_request(req).await {
                            Ok(value) => value,
                            Err(rej) => return rej.into_response(),
                        };

                    let res: Res = f($last_var).await;
                    res.into_response()
                })
            }
        }
    };

    ($(($part_ty:ident, $part_var:ident)),+ ; ($last_ty:ident, $last_var:ident)) => {
        impl<HandlerFn, Fut, Res, $($part_ty,)+ $last_ty> $crate::handler::Handler<($($part_ty,)+ $last_ty,)> for HandlerFn
        where
            HandlerFn: Fn($($part_ty,)+ $last_ty) -> Fut + Clone + $crate::maybe_send::MaybeSend + $crate::maybe_send::MaybeSync + 'static,
            Fut: core::future::Future<Output = Res> + $crate::maybe_send::MaybeSend + 'static,
            Res: $crate::response::IntoResponse + 'static,
            $($part_ty: $crate::extract::FromRequestParts + $crate::maybe_send::MaybeSend + 'static,)+
            $last_ty: $crate::extract::FromRequest + $crate::maybe_send::MaybeSend + 'static,
        {
            fn call(
                &self,
                req: http::Request<h2::RecvStream>,
            ) -> $crate::maybe_send::BoxFuture<'static, http::Response<$crate::body::Body>> {
                let f = self.clone();

                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();

                    $(
                        let $part_var: $part_ty =
                            match <$part_ty as $crate::extract::FromRequestParts>::from_request_parts(&mut parts).await {
                                Ok(value) => value,
                                Err(rej) => return rej.into_response(),
                            };
                    )+

                    let req = http::Request::from_parts(parts, body);

                    let $last_var: $last_ty =
                        match <$last_ty as $crate::extract::FromRequest>::from_request(req).await {
                            Ok(value) => value,
                            Err(rej) => return rej.into_response(),
                        };

                    let res: Res = f($($part_var,)+ $last_var).await;
                    res.into_response()
                })
            }
        }
    };
}

impl_handler!((A, a));
impl_handler!((A, a) ; (B, b));
impl_handler!((A, a), (B, b) ; (C, c));
impl_handler!((A, a), (B, b), (C, c) ; (D, d));
impl_handler!((A, a), (B, b), (C, c), (D, d) ; (E, e));
impl_handler!((A, a), (B, b), (C, c), (D, d), (E, e) ; (F, f));
impl_handler!((A, a), (B, b), (C, c), (D, d), (E, e), (F, f) ; (G, g));
impl_handler!((A, a), (B, b), (C, c), (D, d), (E, e), (F, f), (G, g) ; (H, h));
impl_handler!((A, a), (B, b), (C, c), (D, d), (E, e), (F, f), (G, g), (H, h) ; (I, i));
impl_handler!((A, a), (B, b), (C, c), (D, d), (E, e), (F, f), (G, g), (H, h), (I, i) ; (J, j));
