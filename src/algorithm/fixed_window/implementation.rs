use std::time::Duration;

use tracing::{event, span, Level};

use crate::algorithm::{Limiter, RateLimitError};

use super::FixedWindowStorage;

#[derive(Clone)]
pub struct FixedWindow<S: FixedWindowStorage> {
    size: Duration,
    storage: S,
    max_requests: u64,
}

impl<S: FixedWindowStorage> FixedWindow<S> {
    pub fn new(size: Duration, storage: S, max_requests: u64) -> Self {
        FixedWindow {
            size,
            storage,
            max_requests,
        }
    }
}

impl<S: FixedWindowStorage> Limiter for FixedWindow<S> {
    async fn validate_request(&mut self) -> Result<bool, RateLimitError> {
        let span = span!(Level::INFO, "validate_request");
        let _ = span.enter();
        self.storage.record_fixed_window(self.size).await.unwrap();
        event!(Level::INFO, "updated window");
        let current_count = self.storage.fetch_fixed_window(self.size).await.unwrap();
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
