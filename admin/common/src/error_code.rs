use std::fmt;

use serde::{Deserialize, Serialize};

/// 通用错误码 trait
pub trait ErrorCode {
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;
}

/// 基础错误码枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaseErrorCode {
    // 一级宏观错误码 客户端错误
    ClientError,

    // 二级宏观错误码 用户注册错误
    UserRegisterError,
    UserNameVerifyError,
    UserNameExistError,
    UserNameSensitiveError,
    UserNameSpecialCharacterError,
    PasswordVerifyError,
    PasswordShortError,
    PhoneVerifyError,

    // 二级宏观错误码 系统请求缺少幂等Token
    IdempotentTokenNullError,
    IdempotentTokenDeleteError,

    // 二级宏观错误码 查询参数错误
    SearchAmountExceedsLimit,

    // 添加一个参数无效的错误码
    InvalidParam,

    // 一级宏观错误码 系统执行出错
    ServiceError,
    // 二级宏观错误码 系统执行超时
    ServiceTimeoutError,

    // 一级宏观错误码 调用第三方服务出错
    RemoteError,
}

impl ErrorCode for BaseErrorCode {
    fn code(&self) -> &'static str {
        match self {
            BaseErrorCode::ClientError => "A000001",
            BaseErrorCode::UserRegisterError => "A000100",
            BaseErrorCode::UserNameVerifyError => "A000110",
            BaseErrorCode::UserNameExistError => "A000111",
            BaseErrorCode::UserNameSensitiveError => "A000112",
            BaseErrorCode::UserNameSpecialCharacterError => "A000113",
            BaseErrorCode::PasswordVerifyError => "A000120",
            BaseErrorCode::PasswordShortError => "A000121",
            BaseErrorCode::PhoneVerifyError => "A000151",
            BaseErrorCode::IdempotentTokenNullError => "A000200",
            BaseErrorCode::IdempotentTokenDeleteError => "A000201",
            BaseErrorCode::SearchAmountExceedsLimit => "A000300",
            BaseErrorCode::InvalidParam => "A000400",
            BaseErrorCode::ServiceError => "B000001",
            BaseErrorCode::ServiceTimeoutError => "B000100",
            BaseErrorCode::RemoteError => "C000001",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            BaseErrorCode::ClientError => "用户端错误",
            BaseErrorCode::UserRegisterError => "用户注册错误",
            BaseErrorCode::UserNameVerifyError => "用户名校验失败",
            BaseErrorCode::UserNameExistError => "用户名已存在",
            BaseErrorCode::UserNameSensitiveError => "用户名包含敏感词",
            BaseErrorCode::UserNameSpecialCharacterError => "用户名包含特殊字符",
            BaseErrorCode::PasswordVerifyError => "密码校验失败",
            BaseErrorCode::PasswordShortError => "密码长度不够",
            BaseErrorCode::PhoneVerifyError => "手机格式校验失败",
            BaseErrorCode::IdempotentTokenNullError => "幂等Token为空",
            BaseErrorCode::IdempotentTokenDeleteError => "幂等Token已被使用或失效",
            BaseErrorCode::SearchAmountExceedsLimit => "查询数据量超过最大限制",
            BaseErrorCode::InvalidParam => "无效的请求参数",
            BaseErrorCode::ServiceError => "系统执行出错",
            BaseErrorCode::ServiceTimeoutError => "系统执行超时",
            BaseErrorCode::RemoteError => "调用第三方服务出错",
        }
    }
}

impl fmt::Display for BaseErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

impl std::error::Error for BaseErrorCode {}
