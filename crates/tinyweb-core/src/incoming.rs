use core::pin::Pin;

use crate::io::Io;

pub trait Incoming: Send + 'static {
    type Io: Io;
    type Addr;
    type Error;

    fn accept(
        &mut self,
    ) -> Pin<
        Box<dyn Future<Output = Result<(Self::Io, Self::Addr), Self::Error>> + Send + '_>,
    >;
}
