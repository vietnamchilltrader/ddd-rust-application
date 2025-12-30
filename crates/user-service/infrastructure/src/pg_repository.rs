use crate::UserModel;
use anyhow::Result;
use async_trait::async_trait;
use domain::{
    User,
    repository::UserRepositories,
    value_objects::{UserId, Username},
};
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
        .await?;
        Ok(user.id)
    }
    async fn find_by_id(&self, username: &Username) -> Result<User> {
        let row = sqlx::query_as::<_, UserModel>("SELECT * FROM tbl_users WHERE username = $1")
            .bind(username.as_str())
            .fetch_one(&*self.pool)
            .await?;
        Ok(User::from(row))
    }
}
