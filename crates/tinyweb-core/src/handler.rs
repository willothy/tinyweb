mod erased;

use std::pin::Pin;

use crate::{body::Body, response::IntoResponse};

pub(crate) use erased::{erase_handler, ErasedHandler};

pub trait Handler<T>: Send + Sync + 'static {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> Pin<Box<dyn Future<Output = http::Response<Body>> + Send + 'static>>;
}

impl<F, Fut, Res> Handler<()> for F
where
    F: Fn() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse + 'static,
{
    fn call(
        &self,
        _req: http::Request<h2::RecvStream>,
    ) -> Pin<Box<dyn Future<Output = http::Response<Body>> + Send + 'static>> {
        let f = self.clone();
        Box::pin(async move { f().await.into_response() })
    }
}

macro_rules! impl_handler {
    // 1-arg handler: single arg is extracted from the full request
    (($last_ty:ident, $last_var:ident)) => {
        impl<HandlerFn, Fut, Res, $last_ty> $crate::handler::Handler<($last_ty,)> for HandlerFn
        where
            HandlerFn: Fn($last_ty) -> Fut + Clone + Send + Sync + 'static,
            Fut: core::future::Future<Output = Res> + Send + 'static,
            Res: $crate::response::IntoResponse + 'static,
            $last_ty: $crate::extract::FromRequest + Send + 'static,
        {
            fn call(
                &self,
                req: http::Request<h2::RecvStream>,
            ) -> core::pin::Pin<
                Box<dyn core::future::Future<Output = http::Response<$crate::body::Body>> + Send + 'static>
            > {
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

    // N-arg handler: all but last are parts extractors, last is full request extractor
    ($(($part_ty:ident, $part_var:ident)),+ ; ($last_ty:ident, $last_var:ident)) => {
        impl<HandlerFn, Fut, Res, $($part_ty,)+ $last_ty> $crate::handler::Handler<($($part_ty,)+ $last_ty,)> for HandlerFn
        where
            HandlerFn: Fn($($part_ty,)+ $last_ty) -> Fut + Clone + Send + Sync + 'static,
            Fut: core::future::Future<Output = Res> + Send + 'static,
            Res: $crate::response::IntoResponse + 'static,
            $($part_ty: $crate::extract::FromRequestParts + Send + 'static,)+
            $last_ty: $crate::extract::FromRequest + Send + 'static,
        {
            fn call(
                &self,
                req: http::Request<h2::RecvStream>,
            ) -> core::pin::Pin<
                Box<dyn core::future::Future<Output = http::Response<$crate::body::Body>> + Send + 'static>
            > {
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
