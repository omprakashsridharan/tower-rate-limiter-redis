pub mod fixed_window;
pub mod sliding_log;

pub struct RateLimitError {}

pub trait Limiter: Send + Sync + 'static {
    fn validate_request(
        &mut self,
    ) -> impl std::future::Future<Output = Result<bool, RateLimitError>> + Send;
}
