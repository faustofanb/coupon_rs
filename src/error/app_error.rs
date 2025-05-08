use std::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};

use crate::transfer::ResultVO;

use super::error_code::{BaseErrorCode, ErrorCode};

/// 统一异常类型
#[derive(Debug)]
pub enum AppError {
    Client { code: &'static str, message: String },
    Remote { code: &'static str, message: String },
    Service { code: &'static str, message: String },
}

impl AppError {
    pub fn client<E: ErrorCode>(err: E, msg: Option<String>) -> Self {
        AppError::Client {
            code: err.code(),
            message: msg.unwrap_or_else(|| err.message().to_string()),
        }
    }

    pub fn remote<E: ErrorCode>(err: E, msg: Option<String>) -> Self {
        AppError::Remote {
            code: err.code(),
            message: msg.unwrap_or_else(|| err.message().to_string()),
        }
    }

    pub fn service<E: ErrorCode>(err: E, msg: Option<String>) -> Self {
        AppError::Service {
            code: err.code(),
            message: msg.unwrap_or_else(|| err.message().to_string()),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppError::Client { code, .. } => code,
            AppError::Remote { code, .. } => code,
            AppError::Service { code, .. } => code,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            AppError::Client { message, .. } => message,
            AppError::Remote { message, .. } => message,
            AppError::Service { message, .. } => message,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::service(BaseErrorCode::ServiceError, Some(err.to_string()))
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Client { .. } => StatusCode::BAD_REQUEST,
            AppError::Service { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Remote { .. } => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ResultVO::<()>::failure_from_error(self))
    }
}
