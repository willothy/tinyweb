use core::pin::Pin;

use crate::error::BodyError;

pub enum Body {
    Empty,
    Data(bytes::Bytes),
    Stream(
        Pin<
            Box<
                dyn futures_core::Stream<Item = Result<bytes::Bytes, BodyError>>
                    + Send
                    + 'static,
            >,
        >,
    ),
}

impl core::fmt::Debug for Body {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "Body::Empty"),
            Self::Data(data) => f.debug_tuple("Body::Data").field(&data.len()).finish(),
            Self::Stream(_) => write!(f, "Body::Stream(..)"),
        }
    }
}
