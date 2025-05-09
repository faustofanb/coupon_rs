use actix_web::{post, web, Responder};
use common::app_error::AppError;
use common::transfer::ResultVO;
use services::dto::template_req::TemplateSaveReqDto;
use services::template::template_service;
use services::AppState;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/merchant-admin/coupon-template").service(create_template_route));
}

#[post("/create")]
async fn create_template_route(
    req: web::Json<TemplateSaveReqDto>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let rows = template_service()
        .create_template(req.into_inner(), app_state)
        .await?;

    Ok(ResultVO::success_with("模板创建成功", rows))
}
