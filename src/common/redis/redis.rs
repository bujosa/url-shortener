use redis::{AsyncCommands, RedisError};
use serde::{de::DeserializeOwned, Serialize};

/// A Redis client struct that contains a Redis client instance.
#[derive(Clone)]
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
    pub async fn set_json<T: Serialize>(&self, key: String, b: T) -> redis::RedisResult<()> {
        let mut con = self.client.get_async_connection().await?;

        let json = serde_json::to_string(&b).map_err(|err| {
            RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization Error",
                err.to_string(),
            ))
        })?;

        con.set(key, json).await?;

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
