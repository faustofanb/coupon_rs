use crate::services::template_service::create_template;
use crate::transfer::ResultVO;
use crate::transfer::request::template_req::TemplateSaveReqDto;
use crate::{AppState, error::app_error::AppError};
use actix_web::{Responder, post, web};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/merchant-admin/coupon-template").service(create_template_route));
}

#[post("/create")]
async fn create_template_route(
    req: web::Json<TemplateSaveReqDto>,
    app_state: web::Data<AppState<'_>>,
) -> Result<impl Responder, AppError> {
    let rows = create_template(req.into_inner(), app_state).await?;
    Ok(ResultVO::success_with("模板创建成功", rows))
}
