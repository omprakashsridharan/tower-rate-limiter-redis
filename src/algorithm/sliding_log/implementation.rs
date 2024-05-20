use std::time::Duration;

use tracing::{event, span, Level};

use crate::algorithm::Limiter;

use super::SlidingWindowStorage;

#[derive(Clone)]
pub struct SlidingWindow<S: SlidingWindowStorage> {
    size: Duration,
    storage: S,
    max_requests: u64,
}

impl<S: SlidingWindowStorage> SlidingWindow<S> {
    pub fn new(size: Duration, storage: S, max_requests: u64) -> Self {
        SlidingWindow {
            size,
            storage,
            max_requests,
        }
    }
}

impl<S: SlidingWindowStorage> Limiter for SlidingWindow<S> {
    async fn validate_request(&mut self) -> Result<bool, crate::algorithm::RateLimitError> {
        let span = span!(Level::INFO, "validate_request");
        let _ = span.enter();
        self.storage.record_sliding_log(self.size).await.unwrap();
        event!(Level::INFO, "updated window");
        let current_count = self.storage.fetch_sliding_log().await.unwrap();
        event!(Level::INFO, "current count is {}", current_count);
        if current_count <= self.max_requests {
            event!(Level::INFO, "request within limit {}", self.max_requests);
            Ok(true)
        } else {
            event!(Level::INFO, "request exceeded limit {}", self.max_requests);
            Ok(false)
        }
    }
}
