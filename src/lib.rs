use http::{Request, Response};
use std::{future::Future, task::Poll};

use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct RateLimitLayer {}

impl RateLimitLayer {
    pub fn new() -> Self {
        RateLimitLayer {}
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimit<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimit::new(inner)
    }
}

#[derive(Debug, Clone)]
pub struct RateLimit<S> {
    inner: S,
}

impl<S> RateLimit<S> {
    pub fn new(inner: S) -> Self {
        RateLimit { inner }
    }

    pub fn layer() -> RateLimitLayer {
        RateLimitLayer::new()
    }
}

#[pin_project::pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    response_future: F,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for RateLimit<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    ResBody: Default,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let response_future = self.inner.call(req);
        ResponseFuture { response_future }
    }
}

impl<F, B, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: Default,
{
    type Output = Result<Response<B>, E>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        // check if request is within rate limit if not throw rate limit error

        if 1 < 2 {
            let mut res = Response::new(B::default());
            // *res.status_mut() = StatusCode::REQUEST_TIMEOUT;
            return Poll::Ready(Ok(res));
        }

        this.response_future.poll(cx)
    }
}
