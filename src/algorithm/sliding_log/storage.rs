use std::time::{self, SystemTime};

use redis::{aio::MultiplexedConnection, AsyncCommands};
use tracing::{event, span, Level};

use super::SlidingLogStorage;

#[derive(Clone, Debug)]
pub struct SlidingLogRedisStorage {
    conn: MultiplexedConnection,
}

impl SlidingLogRedisStorage {
    pub async fn new(conn_url: &str) -> Self {
        let client = redis::Client::open(conn_url).unwrap();
        let conn = client.get_multiplexed_tokio_connection().await.unwrap();
        SlidingLogRedisStorage { conn }
    }
}

impl SlidingLogStorage for SlidingLogRedisStorage {
    async fn record_sliding_log(
        &mut self,
        size: std::time::Duration,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let span = span!(Level::INFO, "record_sliding_log");
        let _ = span.enter();
        let now = SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
        event!(Level::INFO, "current time is {:?}", now);
        let key_prefix = "key";
        let key = format!("{}", key_prefix);
        event!(Level::INFO, "computed key {}", key);

        let (count,): (u64,) = redis::pipe()
            .atomic()
            .zrembyscore(&key, 0, (now.as_millis() - size.as_millis()) as u64)
            .ignore()
            .zadd(&key, now.as_millis() as u64, now.as_millis() as u64)
            .ignore()
            .zcard(&key)
            .expire(&key, size.as_secs() as i64)
            .ignore()
            .query_async(&mut self.conn)
            .await?;

        event!(Level::INFO, "count {}", count);
        Ok(count)
    }

    async fn fetch_sliding_log(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        let span = span!(Level::INFO, "fetch_sliding_log");
        let _ = span.enter();
        let key_prefix = "key";
        let key = format!("{}", key_prefix);
        event!(Level::INFO, "computed key {}", key);

        let count: u64 = self.conn.zcard(key).await?;
        event!(Level::INFO, "count {}", count);
        Ok(count)
    }
}
