use crate::commands::AddUserCommand;
use base::web::error::AppError;
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
    pub async fn create(&self, cmd: AddUserCommand) -> Result<UserId, AppError> {
        let username = Username::new(&cmd.username)?;
        let username_exists = self.user_repo.find_by_id(&username).await;
        if username_exists.is_ok() {
            return Err(AppError::BadRequest("Username already exists".into()));
        }
        let password_hash = Password::from_plain(&cmd.password)?;
        let email = EmailAddress::new(cmd.email)?;
        let user = User::new(username, password_hash, email);
        let user_id = self
            .user_repo
            .create(user)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        Ok(user_id)
    }
}
