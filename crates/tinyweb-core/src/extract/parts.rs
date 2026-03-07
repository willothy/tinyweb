use crate::{extract::FromRequestParts, maybe_send::BoxFuture};

impl FromRequestParts for http::Method {
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut http::request::Parts,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>> {
        let method = parts.method.clone();
        Box::pin(async move { Ok(method) })
    }
}

impl FromRequestParts for http::Uri {
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut http::request::Parts,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>> {
        let uri = parts.uri.clone();
        Box::pin(async move { Ok(uri) })
    }
}

impl FromRequestParts for http::HeaderMap {
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut http::request::Parts,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>> {
        let headers = parts.headers.clone();
        Box::pin(async move { Ok(headers) })
    }
}
