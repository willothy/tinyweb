use core::fmt;

#[derive(Debug)]
pub enum ServeConnectionError {
    Handshake(h2::Error),
    Accept(h2::Error),
}

impl fmt::Display for ServeConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Handshake(e) => write!(f, "h2 handshake failed: {e}"),
            Self::Accept(e) => write!(f, "h2 accept failed: {e}"),
        }
    }
}

impl std::error::Error for ServeConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Handshake(e) | Self::Accept(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub enum ServeError<E> {
    Accept(E),
}

impl<E: fmt::Display> fmt::Display for ServeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept(e) => write!(f, "accept failed: {e}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for ServeError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Accept(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub struct BodyError(pub Box<dyn std::error::Error + Send + Sync>);

impl fmt::Display for BodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for BodyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.0)
    }
}
