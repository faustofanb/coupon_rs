use actix_web::{Responder, post, web, get, HttpResponse};
use common::app_error::AppError;
use common::transfer::ResultVO;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(greet);
    // cfg.service(web::scope("/api/merchant-admin/coupon-template").service(create_template_route));
}

#[get("/")]
async  fn greet() -> Result<ResultVO<String>, AppError> {
    Ok(ResultVO::success_with_data("Hello World".to_string()))
}

// #[post("/create")]
// async fn create_template_route(
//     req: web::Json<TemplateSaveReqDto>,
//     app_state: web::Data<AppState<'_>>,
// ) -> Result<impl Responder, AppError> {
//     let rows = create_template(req.into_inner(), app_state).await?;
//     Ok(ResultVO::success_with("模板创建成功", rows))
// }
