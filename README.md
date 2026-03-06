# tinyweb

A minimal, proc-macro-free HTTP/2 server framework for Rust.

Built directly on [h2](https://crates.io/crates/h2) with no proc macros —
handler extraction uses declarative macros and standard traits. Runtime and
transport agnostic, with support for both multi-threaded (tokio) and
single-threaded (compio) runtimes.

## Features

- **HTTP/2 only** — built on the `h2` crate
- **No proc macros** — extractors via `FromRequest` / `FromRequestParts` traits
- **Runtime agnostic** — tokio or compio via feature flags
- **Transport agnostic** — TCP, Unix sockets, or any `AsyncRead + AsyncWrite` stream
- **Optional `Send` bounds** — `send` feature for multi-threaded runtimes, omit for single-threaded

## Crate Structure

| Crate | Purpose |
|---|---|
| `tinyweb-core` | Core traits, extractors, router, body types |
| `tinyweb-tokio` | Tokio runtime, TCP/Unix incoming, `futures_io` compat |
| `tinyweb-compio` | Compio runtime, TCP/Unix incoming |
| `tinyweb` | Server loop (`serve`, `serve_connection`), re-exports |

## Quick Start

```toml
[dependencies]
tinyweb = { version = "0.1", features = ["tokio"] }
```

```rust
use tinyweb::{Router, server};
use tinyweb_tokio::{TokioRuntime, TcpIncoming};

async fn hello() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().get("/", hello);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    server::serve(TcpIncoming(listener), router, TokioRuntime)
        .await
        .unwrap();
}
```

## Feature Flags

| Flag | Default | Description |
|---|---|---|
| `send` | off | `Send` bounds on futures/traits for multi-threaded runtimes |
| `tokio` | off | Tokio runtime adapter (implies `send`) |
| `compio` | off | Compio runtime adapter |

## Serving

Two levels of server API:

**`serve`** takes an `Incoming` (stream of connections) and serves each one
concurrently. This is the typical entrypoint.

```rust
// TCP
let listener = tokio::net::TcpListener::bind("0.0.0.0:443").await?;
server::serve(TcpIncoming(listener), router, TokioRuntime).await?;

// Unix socket
let listener = tokio::net::UnixListener::bind("/tmp/app.sock")?;
server::serve(UnixIncoming(listener), router, TokioRuntime).await?;
```

**`serve_connection`** serves a single IO stream directly — no `Incoming` needed.
The IO type just needs `futures_io::AsyncRead + AsyncWrite`.

```rust
let (stream, _addr) = listener.accept().await?;
server::serve_connection(TokioIoCompat::new(stream), router, TokioRuntime).await?;
```

## Handlers

Any async function is a handler. The return type must implement `IntoResponse`.

```rust
async fn index() -> &'static str {
    "hello"
}

async fn json_status() -> http::Response<Body> {
    http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Data(r#"{"ok":true}"#.into()))
        .unwrap()
}
```

Handler arguments are extracted from the request via `FromRequestParts`
(headers, URI, method) or `FromRequest` (consumes the body — must be the last
argument). Up to 10 extractor arguments are supported.

## Router

Builder-style route registration with [matchit](https://crates.io/crates/matchit)
for path matching (supports `{param}` and `{*wildcard}` syntax).

```rust
let router = Router::new()
    .get("/", index)
    .get("/users/{id}", get_user)
    .post("/users", create_user)
    .delete("/users/{id}", delete_user)
    .many(&[Method::GET, Method::HEAD], "/health", health);
```

`Router` implements `Service`, so it plugs directly into `serve` and
`serve_connection`.

## Custom Service

`Service` is the dispatch interface between the server and your application.
Implement it directly for custom dispatch or middleware-like wrapping:

```rust
#[derive(Clone)]
struct MyService;

impl Service for MyService {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        Box::pin(async move {
            // dispatch logic
        })
    }
}

server::serve(incoming, MyService, runtime).await?;
```

## Custom Transport

Implement `Incoming` to use any transport:

```rust
impl Incoming for MyTransport {
    type Io = MyStream; // futures_io::AsyncRead + AsyncWrite
    type Addr = MyAddr;
    type Error = std::io::Error;

    fn accept(&mut self) -> BoxFuture<'_, Result<(Self::Io, Self::Addr), Self::Error>> {
        Box::pin(async move { /* accept a connection */ })
    }
}
```

## Custom Runtime

Implement `Runtime` to use a runtime other than tokio or compio:

```rust
#[derive(Clone)]
struct MyRuntime;

impl Runtime for MyRuntime {
    fn spawn(&self, fut: BoxFuture<'static, ()>) {
        // spawn the future
    }
}
```

## `send` Feature

Controls whether futures and trait objects require `Send + Sync`, via
`MaybeSend` / `MaybeSync` conditional traits in `tinyweb-core`:

- **With `send`** (tokio): `MaybeSend` requires `Send`, `BoxFuture` includes `+ Send`
- **Without `send`** (compio): `MaybeSend` is a no-op, no `Send` bound on `BoxFuture`

`tinyweb-tokio` and `tinyweb-compio` cannot be compiled together — they require
mutually exclusive states of the `send` feature on `tinyweb-core`.
