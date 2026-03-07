//! Tokio runtime adapter for tinyweb.

#![forbid(missing_docs)]

/// Incoming connection types for TCP and Unix sockets.
pub mod incoming;
/// Compatibility wrapper for converting tokio IO types to `futures_io`.
pub mod io_compat;
/// Tokio runtime implementation.
pub mod runtime;

pub use incoming::{TcpIncoming, UnixIncoming};
pub use io_compat::TokioIoCompat;
pub use runtime::TokioRuntime;
