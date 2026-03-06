use crate::{
    body::Body,
    maybe_send::{BoxFuture, MaybeSend, MaybeSync},
};

pub trait Service: Clone + MaybeSend + MaybeSync + 'static {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>>;
}
