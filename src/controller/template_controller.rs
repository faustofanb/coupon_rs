use actix_web::web;
use actix_web::{get, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/merchant-admin/coupon-template")
            .service(greet)
        )    
    ;
}

#[get("/create")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}