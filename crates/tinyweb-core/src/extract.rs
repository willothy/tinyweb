use crate::{
    maybe_send::{BoxFuture, MaybeSend},
    response::IntoResponse,
};

pub trait FromRequestParts: Sized {
    type Rejection: IntoResponse;

    fn from_request_parts(
        parts: &http::request::Parts,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>>;
}

pub trait FromRequest: Sized {
    type Rejection: IntoResponse;

    fn from_request(
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>>;
}

impl<T> FromRequest for T
where
    T: FromRequestParts + MaybeSend + 'static,
{
    type Rejection = <T as FromRequestParts>::Rejection;

    fn from_request(
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>> {
        Box::pin(async move {
            let (parts, _body) = req.into_parts();
            T::from_request_parts(&parts).await
        })
    }
}
