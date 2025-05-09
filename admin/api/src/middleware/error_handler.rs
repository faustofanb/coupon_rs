use actix_web::{
    body::{BoxBody, MessageBody}, // Removed EitherBody as map_into_right_body handles it for ServiceResponse
    dev::ServiceResponse,
    http::StatusCode,
    middleware::ErrorHandlerResponse,
    // ResponseError, // Only needed if you call app_error.status_code() AND it's not in scope via AppError
};
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use common::error_code::ErrorCode;
use common::{
    app_error::{
        AppError,
        BAD_REQUEST_CODE,
        INTERNAL_ERROR_CODE,
        VALIDATION_ERROR_CODE, // Make sure this is defined in app_error.rs
    },
    error_code::BaseErrorCode, // For generic error codes
    transfer::ResultVO,
};
use log::warn;

// Helper to create the final JSON response
// It now takes the original_http_status to ensure the response status is correct.
fn create_final_response(
    req: HttpRequest,
    original_http_status: StatusCode, // Explicitly use the original status for the HTTP response
    app_error_instance: AppError,     // AppError provides the body (code, message)
) -> ActixResult<ErrorHandlerResponse<BoxBody>> {
    let result_vo = ResultVO::<()>::failure_from_error(&app_error_instance);
    // Build HttpResponse with the original_http_status determined by Actix or previous middleware
    let http_response = HttpResponse::build(original_http_status).json(result_vo);
    // map_into_right_body is used because ErrorHandlerResponse expects an EitherBody context
    let new_sr = ServiceResponse::new(req, http_response.map_into_right_body());
    Ok(ErrorHandlerResponse::Response(new_sr))
}

// The single default error handler for all HTTP errors not handled by AppError::ResponseError
pub fn render_default_error<B: MessageBody + 'static>(
    res: ServiceResponse<B>,
) -> ActixResult<ErrorHandlerResponse<BoxBody>> {
    let original_status = res.status();
    // Deconstruct to get the original request and the original response (which might contain an error)
    let (req, orig_resp) = res.into_parts();

    let mut detailed_message: Option<String> = None;
    if let Some(error) = orig_resp.error() {
        // Extract error from the original response if present
        detailed_message = Some(error.to_string());
    }

    warn!(
        "HTTP Error (handled by default_handler): Status={}, Path={}, OriginalError: {:?}",
        original_status,
        req.path(),
        detailed_message
    );

    // Construct AppError based on the original HTTP status code
    // This AppError will primarily be used for generating the JSON body (app code and message)
    let app_error = match original_status {
        StatusCode::INTERNAL_SERVER_ERROR => AppError::internal_error(
            detailed_message.unwrap_or_else(|| "系统内部错误，请稍后重试".to_string()),
        ),
        StatusCode::NOT_FOUND => AppError::not_found("资源", req.path()), // Pass req.path() or relevant part
        StatusCode::UNAUTHORIZED => AppError::unauthorized(
            detailed_message.unwrap_or_else(|| "未授权访问，请检查认证信息".to_string()),
        ),
        StatusCode::FORBIDDEN => {
            AppError::forbidden(detailed_message.unwrap_or_else(|| "禁止访问此资源".to_string()))
        }
        StatusCode::BAD_REQUEST => AppError::Client {
            // Using Client variant directly
            code: VALIDATION_ERROR_CODE, // Defined in your app_error.rs
            message: detailed_message.unwrap_or_else(|| "请求参数无效或格式错误".to_string()),
        },
        StatusCode::METHOD_NOT_ALLOWED => AppError::Client {
            code: BAD_REQUEST_CODE, // Or a more specific app code like "A000405"
            message: format!("方法 {} 不被允许访问路径 {}", req.method(), req.path()),
        },
        // Generic client errors (4xx)
        s if s.is_client_error() => AppError::Client {
            code: BaseErrorCode::ClientError.code(), // Generic client error app code
            message: detailed_message
                .unwrap_or_else(|| format!("客户端请求错误，状态码: {}", s.as_u16())),
        },
        // Generic server errors (5xx) - other than 500 which is handled above
        s if s.is_server_error() => AppError::Service {
            code: BaseErrorCode::ServiceError.code(), // Generic service error app code
            message: detailed_message
                .unwrap_or_else(|| format!("服务器端错误，状态码: {}", s.as_u16())),
        },
        // Fallback for any other status code not covered
        _ => AppError::Service {
            code: INTERNAL_ERROR_CODE,
            message: format!("发生未预期的HTTP错误，状态码: {}", original_status),
        },
    };

    create_final_response(req, original_status, app_error)
}
