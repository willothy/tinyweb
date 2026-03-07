use crate::{error::BodyError, maybe_send::BoxStream};

/// HTTP response body.
pub enum Body {
    /// No body.
    Empty,
    /// A complete body buffered in memory.
    Data(bytes::Bytes),
    /// A streaming body.
    Stream(BoxStream<Result<bytes::Bytes, BodyError>>),
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
