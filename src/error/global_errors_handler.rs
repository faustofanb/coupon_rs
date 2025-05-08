use crate::{
    error::{app_error::AppError, error_code::BaseErrorCode},
    transfer::ResultVO,
};
use actix_web::{
    Error, HttpResponse,
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use log::error;
use std::future::{Ready, ready};

// 定义错误处理中间件
pub struct ErrorHandler;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorHandlerMiddleware { service }))
    }
}

pub struct ErrorHandlerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            // 这里我们可以处理从控制器返回的错误
            let res = match fut.await {
                Ok(res) => res,
                Err(err) => {
                    // 这里可以处理错误，但我们已经在 AppError 的 ResponseError 实现中处理了
                    // 所以这里只需要返回错误即可，会被 actix-web 自动处理
                    return Err(err);
                }
            };
            Ok(res)
        })
    }
}

// 专门用于处理 panic 的中间件，使用 actix_web 提供的线程安全封装
pub struct PanicHandler;

impl<S, B> Transform<S, ServiceRequest> for PanicHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Transform = PanicHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PanicHandlerMiddleware {
            service: std::sync::Arc::new(service),
        }))
    }
}

pub struct PanicHandlerMiddleware<S> {
    service: std::sync::Arc<S>,
}

impl<S, B> Service<ServiceRequest> for PanicHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let method = req.method().clone();
        let path = req.path().to_string();

        let (http_req, payload) = req.into_parts();
        let req_method = http_req.method().clone();
        let req_path = http_req.path().to_string();

        Box::pin(async move {
            let req = ServiceRequest::from_parts(http_req.clone(), payload);

            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| service.call(req)));

            match res {
                Ok(future) => match future.await {
                    Ok(res) => Ok(res.map_into_boxed_body()),
                    Err(err) => {
                        error!("Request failed: {} {}, error: {}", method, path, err);

                        if let Some(app_error) = err.as_error::<AppError>() {
                            let app_error_cloned = app_error;

                            let response = HttpResponse::InternalServerError()
                                .content_type("application/json")
                                .json(ResultVO::<()>::failure_from_error(app_error_cloned));

                            Ok(ServiceResponse::new(http_req.clone(), response)
                                .map_into_boxed_body())
                        } else {
                            let app_error = AppError::service(
                                BaseErrorCode::ServiceError,
                                Some(format!("服务异常: {}", err)),
                            );

                            let response = HttpResponse::InternalServerError()
                                .content_type("application/json")
                                .json(ResultVO::<()>::failure_from_error(&app_error));

                            Ok(ServiceResponse::new(http_req.clone(), response)
                                .map_into_boxed_body())
                        }
                    }
                },
                Err(panic) => {
                    let panic_message = if let Some(s) = panic.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = panic.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "未知 panic 错误".to_string()
                    };

                    error!(
                        "服务发生 panic: {} {}, error: {}",
                        req_method, req_path, panic_message
                    );

                    let app_error = AppError::service(
                        BaseErrorCode::ServiceError,
                        Some(format!("服务崩溃: {}", panic_message)),
                    );

                    let response = HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .json(ResultVO::<()>::failure_from_error(&app_error));

                    Ok(ServiceResponse::new(http_req, response).map_into_boxed_body())
                }
            }
        })
    }
}
