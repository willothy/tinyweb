use tinyweb::{Body, Layer, Router, Service, server};
use tinyweb_core::maybe_send::BoxFuture;
use tinyweb_tokio::{TcpIncoming, TokioRuntime};

async fn index() -> &'static str {
    "public"
}

async fn admin() -> &'static str {
    "admin only"
}

async fn dashboard() -> &'static str {
    "admin dashboard"
}

#[derive(Clone)]
struct LogLayer;

impl<S: Service> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService { inner }
    }
}

#[derive(Clone)]
struct LogService<S> {
    inner: S,
}

impl<S: Service> Service for LogService<S> {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        let inner = self.inner.clone();
        Box::pin(async move {
            let response = inner.call(req).await;
            println!("{method} {path} -> {}", response.status());
            response
        })
    }
}

#[derive(Clone)]
struct AuthLayer;

impl<S: Service> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(self, inner: S) -> Self::Service {
        AuthService { inner }
    }
}

#[derive(Clone)]
struct AuthService<S> {
    inner: S,
}

impl<S: Service> Service for AuthService<S> {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        let has_auth = req.headers().contains_key("authorization");
        let inner = self.inner.clone();
        Box::pin(async move {
            if !has_auth {
                return http::Response::builder()
                    .status(401)
                    .body(Body::Data("unauthorized".into()))
                    .unwrap();
            }
            inner.call(req).await
        })
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        // these two routes require auth
        .get("/admin", admin)
        .get("/admin/dashboard", dashboard)
        .layer(AuthLayer)
        // this route does not
        .get("/", index)
        // logging on everything
        .layer(LogLayer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:8080");
    server::serve(TcpIncoming(listener), router, TokioRuntime)
        .await
        .unwrap();
}
