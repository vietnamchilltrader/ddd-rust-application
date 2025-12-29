use crate::UserModel;
use anyhow::Result;
use async_trait::async_trait;
use domain::{User, repository::UserRepositories, value_objects::UserId};
use sqlx::PgPool;
use std::sync::Arc;

pub struct PgUserRepository {
    pool: Arc<PgPool>,
}

impl PgUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositories for PgUserRepository {
    async fn create(&self, user: User) -> Result<UserId> {
        let user_model = UserModel::from(user.clone());
        sqlx::query(
            "INSERT INTO tbl_users (id, username, password, email)  VALUES ($1, $2, $3, $4)",
        )
        .bind(user_model.id)
        .bind(user_model.username)
        .bind(user_model.password)
        .bind(user_model.email)
        .execute(&*self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to insert user: {}", e))?;
        Ok(user.id)
    }
}
