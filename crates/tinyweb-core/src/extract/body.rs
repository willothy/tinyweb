use crate::{
    body::Body,
    extract::{collect_body, FromRequest},
    maybe_send::BoxFuture,
    response::IntoResponse,
};

pub struct BodyReadError(pub h2::Error);

impl std::fmt::Display for BodyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to read request body: {}", self.0)
    }
}

impl IntoResponse for BodyReadError {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::Data(self.to_string().into()))
            .expect("valid response")
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
