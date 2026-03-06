use core::pin::Pin;

#[cfg(feature = "send")]
pub trait MaybeSend: Send {}

#[cfg(not(feature = "send"))]
pub trait MaybeSend {}

#[cfg(feature = "send")]
impl<T: Send> MaybeSend for T {}
#[cfg(not(feature = "send"))]
impl<T> MaybeSend for T {}

#[cfg(feature = "send")]
pub trait MaybeSync: Sync {}

#[cfg(not(feature = "send"))]
pub trait MaybeSync {}

#[cfg(feature = "send")]
impl<T: Sync> MaybeSync for T {}
#[cfg(not(feature = "send"))]
impl<T> MaybeSync for T {}

#[cfg(feature = "send")]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
#[cfg(not(feature = "send"))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

#[cfg(feature = "send")]
pub type BoxStream<T> = Pin<Box<dyn futures_core::Stream<Item = T> + Send + 'static>>;
#[cfg(not(feature = "send"))]
pub type BoxStream<T> = Pin<Box<dyn futures_core::Stream<Item = T> + 'static>>;
