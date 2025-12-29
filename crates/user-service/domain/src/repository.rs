use anyhow::Result;
use async_trait::async_trait;

use crate::{User, value_objects::UserId};

#[async_trait]
pub trait UserRepositories: Send + Sync {
    async fn create(&self, user: User) -> Result<UserId>;
}
