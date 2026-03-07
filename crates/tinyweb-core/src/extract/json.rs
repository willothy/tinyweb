use crate::{
    body::Body,
    extract::{collect_body, FromRequest},
    maybe_send::{BoxFuture, MaybeSend},
    response::IntoResponse,
};

pub struct Json<T>(pub T);

pub enum JsonRejection {
    BodyRead(h2::Error),
    Deserialize(serde_json::Error),
}

impl std::fmt::Display for JsonRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BodyRead(e) => write!(f, "failed to read request body: {e}"),
            Self::Deserialize(e) => write!(f, "invalid JSON: {e}"),
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
                .body(Body::Data(e.to_string().into()))
                .expect("valid response"),
        }
    }
}
