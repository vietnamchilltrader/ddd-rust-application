use crate::commands::AddUserCommand;
use domain::repository::UserRepositories;
use domain::{
    User,
    value_objects::{EmailAddress, Password, UserId, Username},
};
use std::sync::Arc;

pub struct UserApplicationService<R: UserRepositories> {
    user_repo: Arc<R>,
}

impl<R: UserRepositories> UserApplicationService<R> {
    pub fn new(user_repo: Arc<R>) -> Self {
        Self { user_repo }
    }
    pub async fn create(&self, cmd: AddUserCommand) -> anyhow::Result<UserId> {
        let username =
            Username::new(&cmd.username).map_err(|e| anyhow::anyhow!("Invalid username: {}", e))?;
        let password_hash = Password::from_plain(&cmd.password)
            .map_err(|e| anyhow::anyhow!("Invalid password: {}", e))?;
        let email =
            EmailAddress::new(cmd.email).map_err(|e| anyhow::anyhow!("Invalid email: {}", e))?;

        let user = User::new(username, password_hash, email);
        let user_id = self.user_repo.create(user).await?;
        Ok(user_id)
    }
}
