use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::web::response::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, code) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg, "BAD_REQUEST"),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, "NOT_FOUND"),
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
                "INTERNAL_SERVER_ERROR",
            ),
        };
        ApiResponse::<()>::error(status.as_u16(), message, code.to_string()).into_response()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::InternalServerError(msg) => write!(f, "{}", msg),
        }
    }
}
