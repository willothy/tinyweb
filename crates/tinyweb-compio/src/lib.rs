//! Compio runtime adapter for tinyweb.

#![forbid(missing_docs)]

/// Incoming connection types for TCP and Unix sockets.
pub mod incoming;
/// Compio runtime implementation.
pub mod runtime;

pub use incoming::{TcpIncoming, UnixIncoming};
pub use runtime::CompioRuntime;
