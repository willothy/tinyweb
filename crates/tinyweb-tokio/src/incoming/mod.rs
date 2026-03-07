/// TCP incoming connections.
pub mod tcp;
/// Unix socket incoming connections.
pub mod unix;

pub use tcp::TcpIncoming;
pub use unix::UnixIncoming;
