use crate::dto::template_req::TemplateSaveReqDto;
use crate::AppState;
use actix_web::web::Data;
use common::app_error::AppError;
use data::{dao::template::template_dao, entity::template};
use log::{error, info};
use once_cell::sync::Lazy;
use sea_orm::prelude::async_trait::async_trait;

#[async_trait]
pub trait TemplateService: Send + Sync {
    async fn create_template(
        &self,
        req: TemplateSaveReqDto,
        app_state: Data<AppState>,
    ) -> Result<i64, AppError>;
}

pub struct TemplateServiceImpl;

#[async_trait]
impl TemplateService for TemplateServiceImpl {
    /// 创建优惠券模板
    ///
    /// 此方法接收客户端的请求DTO，转换为实体模型，并通过DAO层保存到数据库
    ///
    /// # 参数
    /// * `req` - 优惠券模板创建请求DTO
    /// * `app_state` - 应用程序状态，包含数据库连接
    ///
    /// # 返回
    /// * `Result<u64, AppError>` - 成功时返回创建的模板ID，失败时返回错误
    async fn create_template(
        &self,
        req: TemplateSaveReqDto,
        app_state: Data<AppState>,
    ) -> Result<i64, AppError> {
        // 使用 From trait 转换请求DTO为数据库模型
        let template_model = template::Model::try_from(req)?;

        // 获取数据库连接
        let db = &app_state.database;

        // 获取 DAO 实例并调用方法
        let dao = template_dao();

        // 使用DAO保存模板
        match dao.create(db, &template_model).await {
            Ok(created_model) => {
                info!(
                    "创建优惠券模板成功, 模板ID: {}, 名称: {}",
                    created_model.id, created_model.name
                );
                Ok(created_model.id)
            }
            Err(err) => {
                error!("创建优惠券模板失败: {}", err);
                Err(AppError::from(err))
            }
        }
    }
}

static TEMPLATE_SERVICE: Lazy<TemplateServiceImpl> = Lazy::new(|| TemplateServiceImpl);

pub fn template_service() -> &'static dyn TemplateService {
    &*TEMPLATE_SERVICE
}
