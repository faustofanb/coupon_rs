use crate::entity::template::{ActiveModel, Model};
use once_cell::sync::Lazy;
use sea_orm::prelude::async_trait::async_trait;
// 修改：使用sync版本
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

// 定义 TemplateDao 特征，添加async_trait
#[async_trait]
pub trait TemplateDao: Send + Sync {
    /// 创建新的优惠券模板
    async fn create(&self, db: &DatabaseConnection, model: &Model) -> Result<Model, DbErr>;
}

/// 优惠券模板数据访问对象实现
pub struct TemplateDaoImpl;

// 实现 TemplateDao 特征，添加async_trait
#[async_trait]
impl TemplateDao for TemplateDaoImpl {
    /// 创建新的优惠券模板
    async fn create(&self, db: &DatabaseConnection, model: &Model) -> Result<Model, DbErr> {
        let active_model: ActiveModel = model.clone().into();

        active_model.insert(db).await
    }
}

// 使用线程安全的Lazy声明单例实例
static TEMPLATE_DAO: Lazy<TemplateDaoImpl> = Lazy::new(|| TemplateDaoImpl);

// 获取 DAO 实例的函数，返回类型为 trait 对象
pub fn template_dao() -> &'static dyn TemplateDao {
    &*TEMPLATE_DAO
}
