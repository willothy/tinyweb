use crate::maybe_send::MaybeSend;

/// Trait alias for types that can be used as transport IO.
pub trait Io: MaybeSend + Unpin + 'static + futures_io::AsyncRead + futures_io::AsyncWrite {}

impl<T> Io for T where
    T: MaybeSend + Unpin + 'static + futures_io::AsyncRead + futures_io::AsyncWrite
{
}
