use std::{error::Error, time::Duration};

pub mod implementation;
pub mod storage;
pub trait FixedWindowStorage: Send + Sync + 'static {
    fn record_fixed_window(
        &mut self,
        size: Duration,
    ) -> impl std::future::Future<Output = Result<u64, Box<dyn Error>>> + Send;

    fn fetch_fixed_window(
        &mut self,
        size: Duration,
    ) -> impl std::future::Future<Output = Result<u64, Box<dyn Error>>> + Send;
}
