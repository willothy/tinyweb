use crate::{
    io::Io,
    maybe_send::{BoxFuture, MaybeSend},
};

/// A source of incoming connections.
pub trait Incoming: MaybeSend + 'static {
    /// The IO type for accepted connections.
    type Io: Io;
    /// The address type for accepted connections.
    type Addr;
    /// The error type for accept failures.
    type Error;

    /// Accept the next connection.
    fn accept(&mut self) -> BoxFuture<'_, Result<(Self::Io, Self::Addr), Self::Error>>;
}
