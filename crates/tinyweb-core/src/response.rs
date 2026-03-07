use crate::{body::Body, maybe_send::MaybeSend};

/// Convert a type into an HTTP response.
pub trait IntoResponse: MaybeSend + 'static {
    /// Perform the conversion.
    fn into_response(self) -> http::Response<Body>;
}

impl IntoResponse for http::Response<Body> {
    fn into_response(self) -> http::Response<Body> {
        self
    }
}

impl IntoResponse for std::convert::Infallible {
    fn into_response(self) -> http::Response<Body> {
        match self {}
    }
}

impl IntoResponse for () {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Body::Empty)
            .expect("valid response")
    }
}

impl IntoResponse for http::StatusCode {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(self)
            .body(Body::Empty)
            .expect("valid response")
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(Body::Data(self.into()))
            .expect("valid response")
    }
}

impl IntoResponse for String {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(Body::Data(self.into()))
            .expect("valid response")
    }
}

impl IntoResponse for bytes::Bytes {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/octet-stream")
            .body(Body::Data(self))
            .expect("valid response")
    }
}

impl<T: IntoResponse, E: IntoResponse> IntoResponse for Result<T, E> {
    fn into_response(self) -> http::Response<Body> {
        match self {
            Ok(v) => v.into_response(),
            Err(e) => e.into_response(),
        }
    }
}
