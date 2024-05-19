use std::time::{self, SystemTime};

use redis::{aio::MultiplexedConnection, AsyncCommands};

use super::FixedWindowStorage;

#[derive(Clone)]
pub struct FixedWindowRedisStorage {
    conn: MultiplexedConnection,
}

impl FixedWindowRedisStorage {
    pub async fn new(conn_url: &str) -> Self {
        let client = redis::Client::open(conn_url).unwrap();
        let conn = client.get_multiplexed_tokio_connection().await.unwrap();
        FixedWindowRedisStorage { conn }
    }
}

impl FixedWindowStorage for FixedWindowRedisStorage {
    async fn record_fixed_window(
        &mut self,
        size: std::time::Duration,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(time::UNIX_EPOCH)?;
        let window = (now.as_secs() / size.as_secs()) * size.as_secs();
        let key_prefix = "key";
        let key = format!("{}:{}", key_prefix, window);

        let (count,): (u64,) = redis::pipe()
            .atomic()
            .incr(&key, 1)
            .expire(&key, size.as_secs() as i64)
            .ignore()
            .query_async(&mut self.conn)
            .await?;
        print!("count {}", count);

        Ok(count)
    }

    async fn fetch_fixed_window(
        &mut self,
        size: time::Duration,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
        let window = (now.as_secs() / size.as_secs()) * size.as_secs();
        let key_prefix = "key";
        let key = format!("{}:{}", key_prefix, window);

        let count: u64 = self.conn.get(key).await?;
        Ok(count)
    }
}
