use crate::{
    body::Body,
    extract::{collect_body, FromRequest},
    maybe_send::{BoxFuture, MaybeSend},
    response::IntoResponse,
};

/// JSON extractor and responder.
///
/// As an extractor, deserializes the request body from JSON.
/// As a responder, serializes the inner value to a JSON response
/// with `Content-Type: application/json`.
pub struct Json<T>(pub T);

/// Rejection type for the [`Json`] extractor.
pub enum JsonRejection {
    /// Failed to read the request body.
    BodyRead(h2::Error),
    /// Failed to deserialize the request body as JSON.
    Deserialize(serde_json::Error),
}

impl std::fmt::Debug for JsonRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BodyRead(e) => f.debug_tuple("BodyRead").field(e).finish(),
            Self::Deserialize(e) => f.debug_tuple("Deserialize").field(e).finish(),
        }
    }
}

impl std::fmt::Display for JsonRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BodyRead(e) => write!(f, "failed to read request body: {e}"),
            Self::Deserialize(e) => write!(f, "invalid JSON: {e}"),
        }
    }
}

impl std::error::Error for JsonRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BodyRead(e) => Some(e),
            Self::Deserialize(e) => Some(e),
        }
    }
}

impl IntoResponse for JsonRejection {
    fn into_response(self) -> http::Response<Body> {
        let status = match &self {
            Self::BodyRead(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::Deserialize(_) => http::StatusCode::UNPROCESSABLE_ENTITY,
        };
        http::Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(Body::Data(self.to_string().into()))
            .expect("valid response")
    }
}

impl<T: serde::de::DeserializeOwned + MaybeSend + 'static> FromRequest for Json<T> {
    type Rejection = JsonRejection;

    fn from_request(
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, Result<Self, Self::Rejection>> {
        Box::pin(async move {
            let (_parts, body) = req.into_parts();
            let bytes = collect_body(body).await.map_err(JsonRejection::BodyRead)?;
            serde_json::from_slice(&bytes)
                .map(Json)
                .map_err(JsonRejection::Deserialize)
        })
    }
}

impl<T: serde::Serialize + MaybeSend + 'static> IntoResponse for Json<T> {
    fn into_response(self) -> http::Response<Body> {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => http::Response::builder()
                .status(http::StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::Data(bytes.into()))
                .expect("valid response"),
            Err(e) => http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(Body::Data(e.to_string().into()))
                .expect("valid response"),
        }
    }
}
