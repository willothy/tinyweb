use std::sync::Arc;

use crate::{
    body::Body,
    handler::Route,
    layer::Layer,
    maybe_send::BoxFuture,
    service::{IntoService, Service},
};

struct RouteId(usize);

struct RouterInner {
    routers: std::collections::HashMap<http::Method, matchit::Router<RouteId>>,
    routes: Vec<Route>,
}

/// HTTP request router with path-based dispatch.
///
/// Routes are registered via builder methods and matched using
/// [matchit](https://crates.io/crates/matchit) (`{param}` and `{*wildcard}` syntax).
#[derive(Clone)]
pub struct Router {
    inner: Arc<RouterInner>,
}

impl Router {
    /// Create an empty router.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RouterInner {
                routers: std::collections::HashMap::new(),
                routes: Vec::new(),
            }),
        }
    }

    fn inner_mut(&mut self) -> &mut RouterInner {
        Arc::get_mut(&mut self.inner).expect("Router is shared; cannot modify after cloning")
    }

    /// Register a route for the given HTTP method and path.
    pub fn route<S: IntoService<T>, T>(
        self,
        method: http::Method,
        path: &str,
        service: S,
    ) -> Self {
        self.route_erased(method, path, Route::new(service.into_service()))
    }

    fn route_erased(mut self, method: http::Method, path: &str, route: Route) -> Self {
        let inner = self.inner_mut();
        let id = RouteId(inner.routes.len());

        inner.routes.push(route);

        let router = inner
            .routers
            .entry(method)
            .or_insert_with(matchit::Router::new);

        if let Err(e) = router.insert(path, id) {
            panic!("Error configuring router: {}", e);
        }

        self
    }

    /// Apply a middleware layer to all routes registered so far.
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: Layer<Route> + Clone,
        L::Service: Service,
    {
        let inner = self.inner_mut();
        inner.routes = inner
            .routes
            .drain(..)
            .map(|route| Route::new(layer.clone().layer(route)))
            .collect();
        self
    }

    /// Register a `GET` route.
    pub fn get<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::GET, path, service)
    }

    /// Register a `POST` route.
    pub fn post<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::POST, path, service)
    }

    /// Register a `PUT` route.
    pub fn put<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::PUT, path, service)
    }

    /// Register a `DELETE` route.
    pub fn delete<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::DELETE, path, service)
    }

    /// Register a `PATCH` route.
    pub fn patch<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::PATCH, path, service)
    }

    /// Register a `HEAD` route.
    pub fn head<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::HEAD, path, service)
    }

    /// Register an `OPTIONS` route.
    pub fn options<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::OPTIONS, path, service)
    }

    /// Register a `TRACE` route.
    pub fn trace<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::TRACE, path, service)
    }

    /// Register a `CONNECT` route.
    pub fn connect<S: IntoService<T>, T>(self, path: &str, service: S) -> Self {
        self.route(http::Method::CONNECT, path, service)
    }

    /// Register a route for all HTTP methods.
    pub fn any<S: IntoService<T>, T>(mut self, path: &str, service: S) -> Self {
        let route = Route::new(service.into_service());
        for method in &[
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::PATCH,
            http::Method::HEAD,
            http::Method::OPTIONS,
            http::Method::TRACE,
            http::Method::CONNECT,
        ] {
            self = self.route_erased(method.clone(), path, route.clone());
        }
        self
    }

    /// Register a route for multiple HTTP methods.
    pub fn many<S: IntoService<T>, T>(
        mut self,
        methods: &[http::Method],
        path: &str,
        service: S,
    ) -> Self {
        let route = Route::new(service.into_service());
        for method in methods {
            self = self.route_erased(method.clone(), path, route.clone());
        }
        self
    }
}

impl RouterInner {
    async fn dispatch(&self, req: http::Request<h2::RecvStream>) -> http::Response<Body> {
        let method = req.method();
        let path = req.uri().path();

        let matched = self
            .routers
            .get(method)
            .and_then(|router| router.at(path).ok());

        if let Some(matched) = matched {
            let route_id = matched.value;
            let route = &self.routes[route_id.0];

            return route.call(req).await;
        }

        http::Response::builder()
            .status(404)
            .body(Body::Empty)
            .expect("simple 404 should not fail")
    }
}

impl Service for Router {
    fn call(&self, req: http::Request<h2::RecvStream>) -> BoxFuture<'static, http::Response<Body>> {
        let inner = Arc::clone(&self.inner);
        Box::pin(async move { inner.dispatch(req).await })
    }
}
