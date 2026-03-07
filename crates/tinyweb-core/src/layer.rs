use crate::service::Service;

pub trait Layer<S: Service> {
    type Service: Service;

    fn layer(self, inner: S) -> Self::Service;
}
