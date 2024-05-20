use std::{sync::Arc, time::Duration};

use axum::{routing::get, Router};
use tokio::{net::TcpListener, sync::RwLock};
use tower::ServiceBuilder;
use tower_rate_limiter_redis::{
    algorithm::{
        fixed_window::{implementation::FixedWindow, storage::FixedWindowRedisStorage},
        sliding_log::{implementation::SlidingLog, storage::SlidingLogRedisStorage},
    },
    middleware::RateLimitLayer,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // Fixed Window
    let fixed_window_redis_storage = FixedWindowRedisStorage::new("redis://127.0.0.1:6379/").await;
    let _fixed_window_limiter =
        FixedWindow::new(Duration::from_secs(60), fixed_window_redis_storage, 3);

    // Sliding log
    let sliding_log_redis_storage = SlidingLogRedisStorage::new("redis://127.0.0.1:6379/").await;
    let sliding_log_limiter = SlidingLog::new(Duration::from_secs(5), sliding_log_redis_storage, 1);

    let middlewares = ServiceBuilder::new().layer(RateLimitLayer::new(Arc::new(RwLock::new(
        sliding_log_limiter,
    ))));

    let app = Router::new().route("/", get(root)).layer(middlewares);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
