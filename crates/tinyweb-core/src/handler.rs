mod erased;

use crate::{
    body::Body,
    maybe_send::{BoxFuture, MaybeSend, MaybeSync},
    response::IntoResponse,
};

pub(crate) use erased::{erase_handler, ErasedHandler};

pub trait Handler<T>: MaybeSend + MaybeSync + 'static {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>>;
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
