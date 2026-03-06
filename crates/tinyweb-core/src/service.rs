use core::pin::Pin;

use crate::body::Body;

pub trait Service: Clone + Send + Sync + 'static {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> Pin<Box<dyn Future<Output = http::Response<Body>> + Send + 'static>>;
}
