use crate::maybe_send::MaybeSend;

pub trait Io: MaybeSend + Unpin + 'static + futures_io::AsyncRead + futures_io::AsyncWrite {}

impl<T> Io for T where
    T: MaybeSend + Unpin + 'static + futures_io::AsyncRead + futures_io::AsyncWrite
{
}
