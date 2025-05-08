use chrono::{DateTime, Utc};
use log::{debug, error};

use crate::error::app_error::AppError;
use crate::error::error_code::BaseErrorCode;
use crate::storage::dao::Table;
use crate::storage::entity::template_entity::TemplateDO;

impl Table<'_, TemplateDO> {
    /// 添加新的优惠券模板到数据库
    ///
    /// # Arguments
    ///
    /// * `req` - 要保存的优惠券模板数据
    ///
    /// # Returns
    ///
    /// * `Result<u64, AppError>` - 成功时返回影响的行数，失败时返回应用错误
    pub async fn add_template(&self, req: &TemplateDO) -> Result<u64, AppError> {
        // 获取当前的 UTC 时间
        let now: DateTime<Utc> = Utc::now();
        debug!("Adding template with ID: {:?}", req.id);

        sqlx::query(
            r#"
            INSERT INTO t_coupon_template (
                `id`, `name`, `shop_number`, `source`, `target`, `goods`, `type`, 
                `valid_start_time`, `valid_end_time`, `stock`, `receive_rule`, 
                `consume_rule`, `status`, `create_time`, `update_time`, `del_flag`
            ) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&req.id)
        .bind(&req.name)
        .bind(&req.shop_number)
        .bind(&req.source)
        .bind(&req.target)
        .bind(&req.goods)
        .bind(&req.r#type)
        .bind(&req.valid_start_time)
        .bind(&req.valid_end_time)
        .bind(&req.stock)
        .bind(&req.receive_rule)
        .bind(&req.consume_rule)
        .bind(&req.status)
        .bind(&now) // create_time
        .bind(&now) // update_time
        .bind(0) // del_flag，假设默认值为0
        .execute(&*self.pool)
        .await
        .map(|x| x.rows_affected())
        .map_err(|err| {
            error!("Failed to add template: {}", err);
            AppError::service(
                BaseErrorCode::ServiceError,
                Some(format!("添加优惠券模板失败: {}", err)),
            )
        })
    }
}
