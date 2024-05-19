use std::{convert::Infallible, sync::Arc};

use axum::body::Body;
use futures::future::BoxFuture;
use http::{response::Response, Request, StatusCode};
use tokio::sync::RwLock;

use tower::{Layer, Service};

use crate::algorithm::Limiter;

#[derive(Debug, Clone)]
pub struct RateLimitLayer<L: Limiter + Send + 'static + Clone> {
    algorithm: Arc<RwLock<L>>,
}

impl<L: Limiter + Send + 'static + Clone> RateLimitLayer<L> {
    pub fn new(algorithm: Arc<RwLock<L>>) -> Self {
        RateLimitLayer { algorithm }
    }
}

impl<S, L: Limiter + Send + 'static + Clone> Layer<S> for RateLimitLayer<L> {
    type Service = RateLimit<S, L>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimit::new(inner, self.algorithm.clone())
    }
}

#[derive(Debug, Clone)]
pub struct RateLimit<S, L: Limiter + Send + 'static + Clone> {
    inner: S,
    algorithm: Arc<RwLock<L>>,
}

impl<S, L: Limiter + Send + 'static + Clone> RateLimit<S, L> {
    pub fn new(inner: S, algorithm: Arc<RwLock<L>>) -> Self {
        RateLimit { inner, algorithm }
    }

    pub fn layer(self) -> RateLimitLayer<L> {
        RateLimitLayer::new(self.algorithm)
    }
}

impl<S, L> Service<Request<Body>> for RateLimit<S, L>
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    L: Limiter + Send + 'static + Clone,
{
    type Response = Response<Body>;

    type Error = Infallible;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut ready_inner = std::mem::replace(&mut self.inner, not_ready_inner);
        let algorithm = self.algorithm.clone();
        Box::pin(async move {
            let unauthorized_response: Response<Body> = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::empty())
                .unwrap();
            let lock = algorithm.read().await;
            match lock.clone().validate_request().await {
                Ok(is_valid) => {
                    drop(lock);
                    if is_valid {
                        let future = ready_inner.call(req);
                        let response: Response<Body> = future.await?;
                        Ok(response)
                    } else {
                        Ok(unauthorized_response)
                    }
                }
                Err(_) => {
                    drop(lock);
                    Ok(unauthorized_response)
                }
            }
        })
    }
}
