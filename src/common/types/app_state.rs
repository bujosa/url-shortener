use std::sync::Arc;

use crate::common::redis::redis::RedisClient;

/// Represents the state of the application, including a Redis client.
#[derive(Clone)]
pub struct AppState {
    /// An Arc-wrapped Redis client.
    pub redis_client: Arc<RedisClient>,
}
