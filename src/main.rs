use std::{sync::Arc, time::Duration};

use axum::{routing::get, Router};
use tokio::{net::TcpListener, sync::RwLock};
use tower::ServiceBuilder;
use tower_rate_limiter_redis::{
    algorithm::fixed_window::{implementation::FixedWindow, storage::FixedWindowRedisStorage},
    middleware::RateLimitLayer,
};

#[tokio::main]
async fn main() {
    let fixed_window_redis_storage = FixedWindowRedisStorage::new("redis://127.0.0.1:6379/").await;
    let fixed_window_limiter =
        FixedWindow::new(Duration::from_secs(10), fixed_window_redis_storage, 3);
    let middlewares = ServiceBuilder::new().layer(RateLimitLayer::new(Arc::new(RwLock::new(
        fixed_window_limiter,
    ))));

    let app = Router::new().route("/", get(root)).layer(middlewares);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
