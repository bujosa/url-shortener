/// RedisClient struct represents a Redis client connection.
///
/// # Example
///
/// ```
/// use crate::common::redis::RedisClient;
///
/// let redis_client = RedisClient::new();
/// redis_client.connection.set("key", "value").unwrap();
/// ```

pub struct RedisClient {
    pub connection: redis::Connection,
}

impl RedisClient {
    /// Creates a new RedisClient instance.
    pub fn new() -> RedisClient {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let connection = client.get_connection().unwrap();
        RedisClient { connection }
    }
}
