use std::{error::Error, time::Duration};

pub mod implementation;
pub mod storage;

pub trait SlidingWindowStorage: Send + Sync + 'static {
    fn record_sliding_log(
        &mut self,
        size: Duration,
    ) -> impl std::future::Future<Output = Result<u64, Box<dyn Error>>> + Send;
    fn fetch_sliding_log(
        &mut self,
    ) -> impl std::future::Future<Output = Result<u64, Box<dyn Error>>> + Send;
}
