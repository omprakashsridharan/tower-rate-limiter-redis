use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_rate_limiter_redis::RateLimitLayer;

#[tokio::main]
async fn main() {
    let middlewares = ServiceBuilder::new().layer(RateLimitLayer::new());

    let app = Router::new().route("/", get(root)).layer(middlewares);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
