use actix_web::web;
use log::info;

use crate::{
    AppState, error::app_error::AppError, storage::entity::template_entity::TemplateDO,
    transfer::request::template_req::TemplateSaveReqDto,
};

pub async fn create_template(
    req: TemplateSaveReqDto,
    app_state: web::Data<AppState<'_>>,
) -> Result<u64, AppError> {
    // 使用From<&TemplateSaveReqDto> trait将请求DTO转换为数据对象
    let template_do = TemplateDO::from(req);

    let rows = app_state
        .database
        .template
        .add_template(&template_do)
        .await?;
    info!(
        "Insert into t_coupon_template success, insert rows: {}.",
        rows
    );

    Ok(rows)
}
