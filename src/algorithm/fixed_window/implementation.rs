use std::time::Duration;

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
        self.storage.record_fixed_window(self.size).await.unwrap();
        let current_count = self.storage.fetch_fixed_window(self.size).await.unwrap();
        if current_count <= self.max_requests {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
