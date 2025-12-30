use anyhow::Result;
use async_trait::async_trait;

use crate::{
    User,
    value_objects::{UserId, Username},
};

#[async_trait]
pub trait UserRepositories: Send + Sync {
    async fn create(&self, user: User) -> Result<UserId>;
    async fn find_by_id(&self, user_id: &Username) -> Result<User>;
}
