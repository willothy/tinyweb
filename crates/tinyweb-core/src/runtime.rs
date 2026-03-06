use core::pin::Pin;

pub trait Runtime: Clone + Send + Sync + 'static {
    fn spawn(
        &self,
        fut: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    );
}
