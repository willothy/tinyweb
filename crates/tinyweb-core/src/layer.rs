use crate::service::Service;

/// Middleware that wraps a [`Service`] to produce a new service.
pub trait Layer<S: Service> {
    /// The wrapped service type.
    type Service: Service;

    /// Wrap the given service.
    fn layer(self, inner: S) -> Self::Service;
}
