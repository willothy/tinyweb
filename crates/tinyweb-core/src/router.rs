use std::sync::Arc;

use crate::{
    body::Body,
    handler::{erase_handler, ErasedHandler, Handler},
    maybe_send::BoxFuture,
    service::Service,
};

struct RouteId(usize);

struct RouterInner {
    routers: std::collections::HashMap<http::Method, matchit::Router<RouteId>>,
    routes: Vec<Arc<dyn ErasedHandler>>,
}

#[derive(Clone)]
pub struct Router {
    inner: Arc<RouterInner>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RouterInner {
                routers: std::collections::HashMap::new(),
                routes: Vec::new(),
            }),
        }
    }

    fn inner_mut(&mut self) -> &mut RouterInner {
        Arc::get_mut(&mut self.inner)
            .expect("Router is shared; cannot modify after cloning")
    }

    pub fn route<H: Handler<T>, T: 'static>(
        self,
        method: http::Method,
        path: &str,
        handler: H,
    ) -> Self {
        self.route_internal(method, path, erase_handler(handler))
    }

    fn route_internal(
        mut self,
        method: http::Method,
        path: &str,
        handler: Arc<dyn ErasedHandler>,
    ) -> Self {
        let inner = self.inner_mut();
        let id = RouteId(inner.routes.len());

        inner.routes.push(handler);

        let router = inner
            .routers
            .entry(method)
            .or_insert_with(matchit::Router::new);

        if let Err(e) = router.insert(path, id) {
            panic!("Error configuring router: {}", e);
        }

        self
    }

    pub fn get<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::GET, path, handler)
    }

    pub fn post<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::POST, path, handler)
    }

    pub fn put<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::PUT, path, handler)
    }

    pub fn delete<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::DELETE, path, handler)
    }

    pub fn patch<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::PATCH, path, handler)
    }

    pub fn head<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::HEAD, path, handler)
    }

    pub fn options<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::OPTIONS, path, handler)
    }

    pub fn trace<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::TRACE, path, handler)
    }

    pub fn connect<H: Handler<T>, T: 'static>(self, path: &str, handler: H) -> Self {
        self.route(http::Method::CONNECT, path, handler)
    }

    pub fn any<H: Handler<T>, T: 'static>(mut self, path: &str, handler: H) -> Self {
        let handler = erase_handler(handler);
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
            self = self.route_internal(method.clone(), path, Arc::clone(&handler));
        }
        self
    }

    pub fn many<H: Handler<T>, T: 'static>(
        mut self,
        methods: &[http::Method],
        path: &str,
        handler: H,
    ) -> Self {
        let handler = erase_handler(handler);
        for method in methods {
            self = self.route_internal(method.clone(), path, Arc::clone(&handler));
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

            return route.call_erased(req).await;
        }

        http::Response::builder()
            .status(404)
            .body(Body::Empty)
            .expect("simple 404 should not fail")
    }
}

impl Service for Router {
    fn call(
        &self,
        req: http::Request<h2::RecvStream>,
    ) -> BoxFuture<'static, http::Response<Body>> {
        let inner = Arc::clone(&self.inner);
        Box::pin(async move { inner.dispatch(req).await })
    }
}
