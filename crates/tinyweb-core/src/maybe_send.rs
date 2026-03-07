use core::pin::Pin;

/// Conditionally requires `Send` when the `send` feature is enabled.
#[cfg(feature = "send")]
pub trait MaybeSend: Send {}

#[cfg(not(feature = "send"))]
/// Unconditionally satisfied when the `send` feature is disabled.
pub trait MaybeSend {}

#[cfg(feature = "send")]
impl<T: Send> MaybeSend for T {}
#[cfg(not(feature = "send"))]
impl<T> MaybeSend for T {}

/// Conditionally requires `Sync` when the `send` feature is enabled.
#[cfg(feature = "send")]
pub trait MaybeSync: Sync {}

#[cfg(not(feature = "send"))]
/// Unconditionally satisfied when the `send` feature is disabled.
pub trait MaybeSync {}

#[cfg(feature = "send")]
impl<T: Sync> MaybeSync for T {}
#[cfg(not(feature = "send"))]
impl<T> MaybeSync for T {}

/// A boxed future that is conditionally `Send`.
#[cfg(feature = "send")]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
/// A boxed future that is not required to be `Send`.
#[cfg(not(feature = "send"))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// A boxed stream that is conditionally `Send`.
#[cfg(feature = "send")]
pub type BoxStream<T> = Pin<Box<dyn futures_core::Stream<Item = T> + Send + 'static>>;
/// A boxed stream that is not required to be `Send`.
#[cfg(not(feature = "send"))]
pub type BoxStream<T> = Pin<Box<dyn futures_core::Stream<Item = T> + 'static>>;
