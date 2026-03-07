//! A minimal, proc-macro-free HTTP/2 server framework.

#![forbid(missing_docs)]

mod io;
/// Server loop for serving connections.
pub mod server;

pub use tinyweb_core::{
    body::Body,
    error::{BodyError, ServeConnectionError, ServeError},
    extract::{BodyStream, FromRequest, FromRequestParts, Json},
    handler::Handler,
    incoming::Incoming,
    io::Io,
    layer::Layer,
    response::IntoResponse,
    router::Router,
    runtime::Runtime,
    service::Service,
};

#[cfg(feature = "tokio")]
pub use tinyweb_tokio;

#[cfg(feature = "compio")]
pub use tinyweb_compio;
