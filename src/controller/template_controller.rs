use actix_web::{HttpResponse, Responder};
use actix_web::{post, web};

use crate::AppState;
use crate::services::template_service;
use crate::transfer::request::template_req::TemplateSaveReqDto;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/merchant-admin/coupon-template").service(create_template));
}

#[post("/create")]
async fn create_template(
    req: web::Json<TemplateSaveReqDto>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let request = req.into_inner();

    let result = template_service::create_template(request, app_state).await;

    match result {
        Ok(rows) => HttpResponse::Accepted().body(format!("insert rows: {}", rows)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
