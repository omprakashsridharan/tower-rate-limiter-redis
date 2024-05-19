pub mod fixed_window;

pub struct RateLimitError {}

pub trait Limiter: Send + Sync + 'static {
    fn validate_request(
        self,
    ) -> impl std::future::Future<Output = Result<bool, RateLimitError>> + Send;
}