use crate::aggregate::User;
use crate::value_objects::UserId;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepositories: Send + Sync {
    async fn create(&self, user: User) -> Result<UserId>;
    async fn find_by_id(&self, user_id: &UserId) -> Result<User>;
}
