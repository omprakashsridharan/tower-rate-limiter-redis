use std::time::Duration;

use super::Limiter;

#[derive(Clone)]
pub struct FixedWindow {
    size: Duration,
}

impl FixedWindow {
    pub fn new(size: Duration) -> Self {
        FixedWindow { size }
    }
}

impl Limiter for FixedWindow {
    async fn validate_request(self) -> Result<bool, super::RateLimitError> {
        Ok(false)
    }
}
