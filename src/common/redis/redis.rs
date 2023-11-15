use redis::AsyncCommands;
use serde::de::DeserializeOwned;

/// A Redis client struct that contains a Redis client instance.
pub struct RedisClient {
    pub client: redis::Client,
}

impl RedisClient {
    /// Creates a new Redis client instance.
    pub fn new() -> RedisClient {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        RedisClient { client }
    }

    /// Sets a JSON value in Redis.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the Redis key.
    /// * `b` - A serde_json::Value that holds the JSON value to be set.
    ///
    /// # Returns
    ///
    /// A RedisResult that indicates whether the operation was successful or not.
    pub async fn set_json(&self, key: String, b: serde_json::Value) -> redis::RedisResult<()> {
        let mut con = self.client.get_async_connection().await?;
        con.set(key, b.to_string()).await?;

        Ok(())
    }

    /// Gets a JSON value from Redis.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the Redis key.
    ///
    /// # Returns
    ///
    /// A RedisResult that contains the JSON value if the operation was successful.
    pub async fn get_json<T: DeserializeOwned>(&self, key: String) -> redis::RedisResult<T> {
        let mut con = self.client.get_async_connection().await?;
        let result: String = con.get(key).await?;
        let value: T = serde_json::from_str(&result).unwrap();

        Ok(value)
    }
}
