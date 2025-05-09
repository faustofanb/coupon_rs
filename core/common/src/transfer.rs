
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::app_error::AppError;
use crate::error_code::{BaseErrorCode, ErrorCode};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultVO<T>
where
    T: Serialize,
{
    pub code: String,
    pub message: String,
    pub data: Option<T>,
    pub request_id: Option<String>,
}

impl<T> ResultVO<T>
where
    T: Serialize,
{
    pub const SUCCESS_CODE: &'static str = "0";

    /// 创建成功返回结果，不含数据
    pub fn success() -> ResultVO<T> {
        ResultVO {
            code: Self::SUCCESS_CODE.to_string(),
            message: String::new(),
            data: None,
            request_id: None,
        }
    }

    /// 创建包含数据的成功返回结果
    pub fn success_with_data(data: T) -> ResultVO<T> {
        ResultVO {
            code: Self::SUCCESS_CODE.to_string(),
            message: String::new(),
            data: Some(data),
            request_id: None,
        }
    }

    /// 创建带消息的成功返回结果
    pub fn success_with_message(message: impl Into<String>) -> ResultVO<T> {
        ResultVO {
            code: Self::SUCCESS_CODE.to_string(),
            message: message.into(),
            data: None,
            request_id: None,
        }
    }

    pub fn success_with(message: impl Into<String>, data: T) -> ResultVO<T> {
        ResultVO {
            code: Self::SUCCESS_CODE.to_string(),
            message: message.into(),
            data: Some(data),
            request_id: None,
        }
    }

    /// 创建失败返回结果，使用默认的错误信息
    pub fn failure() -> ResultVO<T> {
        let error_code = BaseErrorCode::ServiceError;
        ResultVO {
            code: error_code.code().to_string(),
            message: error_code.message().to_string(),
            data: None,
            request_id: None,
        }
    }

    /// 创建失败返回结果，根据给定的异常信息
    pub fn failure_from_error(error: &AppError) -> ResultVO<T> {
        ResultVO {
            code: error.code().to_string(),
            message: error.message().to_string(),
            data: None,
            request_id: None,
        }
    }

    /// 创建失败返回结果，使用指定的错误代码和消息
    pub fn failure_with_code_and_message(code: &str, message: &str) -> ResultVO<T> {
        ResultVO {
            code: code.to_string(),
            message: message.to_string(),
            data: None,
            request_id: None,
        }
    }

    /// 设置请求ID
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// 判断操作是否成功
    pub fn is_success(&self) -> bool {
        self.code == Self::SUCCESS_CODE
    }

    /// 判断操作是否失败
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

// 实现 Responder trait，使 ResultVO 可以直接作为 actix-web 控制器的返回值
impl<T: Serialize + 'static> Responder for ResultVO<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // 将 ResultVO 序列化为 JSON 并包装在 HTTP 响应中
        match serde_json::to_string(&self) {
            Ok(json) => HttpResponse::Ok()
                .content_type("application/json")
                .body(json),
            Err(_) => {
                let error_resp = ResultVO::<()>::failure_with_code_and_message(
                    BaseErrorCode::ServiceError.code(),
                    "结果序列化失败",
                );

                HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(serde_json::to_string(&error_resp).unwrap_or_else(|_| {
                        format!(
                            "{{\"code\":\"{}\",\"message\":\"结果序列化失败\"}}",
                            BaseErrorCode::ServiceError.code()
                        )
                    }))
            }
        }
    }
}
