use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    body::Body,
    extract::{collect_body, FromRequest},
    maybe_send::BoxFuture,
    response::IntoResponse,
};

/// Rejection type returned when reading the request body fails.
pub struct BodyReadError(pub h2::Error);

impl std::fmt::Debug for BodyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for BodyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to read request body: {}", self.0)
    }
}

impl std::error::Error for BodyReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl IntoResponse for BodyReadError {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(Body::Data(self.to_string().into()))
            .expect("valid response")
    }
}

/// Extractor that yields the request body as a `Stream<Item = Result<Bytes, h2::Error>>`.
///
/// Flow control capacity is released automatically as chunks are consumed.
pub struct BodyStream(h2::RecvStream);

impl futures_core::Stream for BodyStream {
    type Item = Result<bytes::Bytes, h2::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.0.poll_data(cx) {
            Poll::Ready(Some(Ok(data))) => {
                if let Err(e) = self.0.flow_control().release_capacity(data.len()) {
                    return Poll::Ready(Some(Err(e)));
                }
                Poll::Ready(Some(Ok(data)))
            }
            other => other,
        }
    }
}

impl FromRequest for BodyStream {
    type Rejection = std::convert::Infallible;

    fn from_request(
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>> {
        let (_parts, body) = req.into_parts();
        Box::pin(async move { Ok(BodyStream(body)) })
    }
}

impl FromRequest for bytes::Bytes {
    type Rejection = BodyReadError;

    fn from_request(
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>> {
        Box::pin(async move {
            let (_parts, body) = req.into_parts();
            collect_body(body).await.map_err(BodyReadError)
        })
    }
}
