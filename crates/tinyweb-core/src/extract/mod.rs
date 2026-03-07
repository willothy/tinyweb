mod body;
mod json;
mod parts;

pub use self::{
    body::{BodyReadError, BodyStream},
    json::{Json, JsonRejection},
};

use crate::{
    maybe_send::{BoxFuture, MaybeSend},
    response::IntoResponse,
};

/// Extract data from request headers, URI, method, or extensions.
///
/// Types implementing this trait can appear as handler arguments
/// in any position except the last (which may be a [`FromRequest`]).
pub trait FromRequestParts: Sized {
    /// The rejection type returned when extraction fails.
    type Rejection: IntoResponse;

    /// Extract from the request parts.
    fn from_request_parts(
        parts: &mut http::request::Parts,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>>;
}

/// Extract data from the full request, consuming the body.
///
/// Types implementing this trait must be the last handler argument,
/// since extraction consumes the request body.
pub trait FromRequest: Sized {
    /// The rejection type returned when extraction fails.
    type Rejection: IntoResponse;

    /// Extract from the full request.
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
            let (mut parts, _body) = req.into_parts();
            T::from_request_parts(&mut parts).await
        })
    }
}

pub(crate) async fn collect_body(
    mut body: h2::RecvStream,
) -> Result<bytes::Bytes, h2::Error> {
    let mut data = bytes::BytesMut::new();
    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        body.flow_control().release_capacity(chunk.len())?;
        data.extend_from_slice(&chunk);
    }
    Ok(data.freeze())
}
