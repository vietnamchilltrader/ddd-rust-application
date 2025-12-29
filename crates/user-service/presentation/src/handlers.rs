use std::sync::Arc;

use application::commands::AddUserCommand;
use application::user_service::UserApplicationService;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use infrastructure::PgUserRepository;
use tracing::info;

use crate::dto::{UserRequest, UserResponse};

pub struct AppState {
    pub user_service: Arc<UserApplicationService<PgUserRepository>>,
}

pub async fn create_user_handler(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<UserRequest>,
) -> impl IntoResponse {
    let command = AddUserCommand {
        username: request.username,
        password: request.password,
        email: request.email,
    };
    let user_id = app_state
        .user_service
        .create(command)
        .await
        .map_err(|e| info!("Error creating user: {}", e))
        .unwrap();

    let response = UserResponse {
        id: user_id.as_str().to_string(),
    };

    (StatusCode::CREATED, Json(response))
}
