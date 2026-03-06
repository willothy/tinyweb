use crate::{
    io::Io,
    maybe_send::{BoxFuture, MaybeSend},
};

pub trait Incoming: MaybeSend + 'static {
    type Io: Io;
    type Addr;
    type Error;

    fn accept(&mut self) -> BoxFuture<'_, Result<(Self::Io, Self::Addr), Self::Error>>;
}
