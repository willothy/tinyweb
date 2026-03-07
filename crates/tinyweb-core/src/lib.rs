//! Core traits, extractors, router, and body types for tinyweb.

#![forbid(missing_docs)]

/// HTTP response body types.
pub mod body;
/// Error types for serving connections and reading bodies.
pub mod error;
/// Request extractors.
pub mod extract;
/// Handler trait and type erasure.
pub mod handler;
/// Incoming connection streams.
pub mod incoming;
/// IO trait for transport abstraction.
pub mod io;
/// Middleware layering.
pub mod layer;
/// Conditional `Send`/`Sync` traits and type aliases.
pub mod maybe_send;
/// Response conversion trait.
pub mod response;
/// HTTP request router.
pub mod router;
/// Runtime abstraction for spawning futures.
pub mod runtime;
/// Service trait for request handling.
pub mod service;
