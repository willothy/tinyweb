use crate::{body::Body, maybe_send::MaybeSend};

pub trait IntoResponse: MaybeSend + 'static {
    fn into_response(self) -> http::Response<Body>;
}

impl IntoResponse for http::Response<Body> {
    fn into_response(self) -> http::Response<Body> {
        self
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> http::Response<Body> {
        http::Response::builder()
            .status(200)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(Body::Data(self.into()))
            .unwrap()
    }
}
