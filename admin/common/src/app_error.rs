use std::fmt;

use crate::transfer::ResultVO;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use log::error;
use sea_orm::DbErr;

use super::error_code::{BaseErrorCode, ErrorCode};

// 定义通用错误码常量
pub const DB_ERROR_CODE: &str = "B000101";
pub const INTERNAL_ERROR_CODE: &str = "B000199";
pub const NOT_FOUND_CODE: &str = "A000404";
pub const VALIDATION_ERROR_CODE: &str = "A000400";
pub const UNAUTHORIZED_CODE: &str = "A000401";
pub const FORBIDDEN_CODE: &str = "A000403";
pub const BAD_REQUEST_CODE: &str = "A000400";
/// 统一异常类型
#[derive(Debug, Clone)]
pub enum AppError {
    Client { code: &'static str, message: String },
    Remote { code: &'static str, message: String },
    Service { code: &'static str, message: String },
}

impl AppError {
    // 现有的方法
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

    // 新增通用错误创建方法
    pub fn internal_error(msg: impl ToString) -> Self {
        AppError::Service {
            code: INTERNAL_ERROR_CODE,
            message: msg.to_string(),
        }
    }

    pub fn db_error(err: impl fmt::Display) -> Self {
        AppError::Service {
            code: DB_ERROR_CODE,
            message: format!("数据库错误: {}", err),
        }
    }

    pub fn validation_error(msg: impl ToString) -> Self {
        AppError::Client {
            code: VALIDATION_ERROR_CODE,
            message: msg.to_string(),
        }
    }

    pub fn not_found(entity: &str, id: impl fmt::Display) -> Self {
        AppError::Client {
            code: NOT_FOUND_CODE,
            message: format!("未找到{}: {}", entity, id),
        }
    }

    pub fn unauthorized(msg: impl ToString) -> Self {
        AppError::Client {
            code: UNAUTHORIZED_CODE,
            message: msg.to_string(),
        }
    }

    pub fn forbidden(msg: impl ToString) -> Self {
        AppError::Client {
            code: FORBIDDEN_CODE,
            message: msg.to_string(),
        }
    }

    pub fn with_context<C: fmt::Display>(self, context: C) -> Self {
        match self {
            AppError::Client { code, message } => AppError::Client {
                code,
                message: format!("{} | 上下文: {}", message, context),
            },
            AppError::Remote { code, message } => AppError::Remote {
                code,
                message: format!("{} | 上下文: {}", message, context),
            },
            AppError::Service { code, message } => AppError::Service {
                code,
                message: format!("{} | 上下文: {}", message, context),
            },
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

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Client { code, .. } => match *code {
                NOT_FOUND_CODE => StatusCode::NOT_FOUND,
                UNAUTHORIZED_CODE => StatusCode::UNAUTHORIZED,
                FORBIDDEN_CODE => StatusCode::FORBIDDEN,
                _ => StatusCode::BAD_REQUEST,
            },
            AppError::Service { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Remote { .. } => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // 记录服务器错误，但不记录客户端错误
        if matches!(self, AppError::Service { .. } | AppError::Remote { .. }) {
            error!("服务错误: {}", self);
        }

        HttpResponse::build(self.status_code()).json(ResultVO::<()>::failure_from_error(self))
    }
}

// 实现常见错误类型转换
impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::db_error(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::internal_error(format!("IO错误: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::client(
            BaseErrorCode::InvalidParam,
            Some(format!("JSON解析错误: {}", err)),
        )
    }
}

// 增加 Result 扩展 trait，提供上下文方法
pub trait ResultExt<T> {
    fn with_context<C: fmt::Display>(self, context: C) -> Result<T, AppError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<AppError>,
{
    fn with_context<C: fmt::Display>(self, context: C) -> Result<T, AppError> {
        self.map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_context(context)
        })
    }
}
