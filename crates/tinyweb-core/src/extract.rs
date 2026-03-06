use crate::response::IntoResponse;

pub trait FromRequestParts: Sized {
    type Rejection: IntoResponse;

    fn from_request_parts(
        parts: &http::request::Parts,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send + 'static;
}

pub trait FromRequest: Sized {
    type Rejection: IntoResponse;

    fn from_request(
        req: http::Request<h2::RecvStream>,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send + 'static;
}

impl<T> FromRequest for T
where
    T: FromRequestParts + Send + 'static,
{
    type Rejection = <T as FromRequestParts>::Rejection;

    fn from_request(
        req: http::Request<h2::RecvStream>,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send + 'static {
        async move {
            let (parts, _body) = req.into_parts();
            T::from_request_parts(&parts).await
        }
    }
}
