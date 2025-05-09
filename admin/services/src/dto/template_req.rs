use crate::auth::SHOP_NUMBER;
use chrono::{DateTime, Utc};
use common::app_error::AppError;
use common::datetime::serde_option_datetime_utc_as_gmt8_string;
use data::entity::template;
use data::enums::{CouponSource, CouponStatus, CouponTarget, CouponType};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
// 移除了 From, 添加了 TryFrom

/// 优惠券模板新增/保存请求 DTO
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TemplateSaveReqDto {
    /// 优惠券名称
    /// 示例: "用户下单满10减3特大优惠"
    pub name: String,

    /// 商店编号
    pub shop_number: Option<i64>,

    /// 优惠券来源
    /// 示例: 0 (店铺券)
    pub source: CouponSource, // 使用枚举

    /// 优惠对象
    /// 示例: 1 (全店通用)
    pub target: CouponTarget, // 使用枚举

    /// 优惠商品编码 (如果 target 是商品专属)
    pub goods: String,

    /// 优惠类型
    /// 示例: 0 (立减券)
    #[serde(rename = "type")]
    pub r#type: CouponType, // 使用枚举

    /// 有效期开始时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub valid_start_time: Option<DateTime<Utc>>,

    /// 有效期结束时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub valid_end_time: Option<DateTime<Utc>>,

    /// 库存
    /// 示例: "200"
    pub stock: i32,

    /// 领取规则 (JSON 字符串)
    /// 示例: "{\"limitPerPerson\":1,\"usageInstructions\":\"3\"}"
    pub receive_rule: String,

    /// 消耗规则 (JSON 字符串)
    /// 示例: "{\"termsOfUse\":10,\"maximumDiscountAmount\":3,...}"
    pub consume_rule: String,
}

// 实现从 TemplateSaveReqDto 到 template::Model 的转换 (使用 TryFrom)
impl TryFrom<TemplateSaveReqDto> for template::Model {
    type Error = AppError; // 指定错误类型

    fn try_from(dto: TemplateSaveReqDto) -> Result<Self, Self::Error> {
        let checked_valid_start_time = match dto.valid_start_time {
            Some(dt) => dt,
            None => {
                return Err(AppError::validation_error(
                    "valid_start_time cannot be empty".to_string(),
                ))
            }
        };
        let checked_valid_end_time = match dto.valid_end_time {
            Some(dt) => dt,
            None => {
                return Err(AppError::validation_error(
                    "valid_end_time cannot be empty".to_string(),
                ));
            }
        };

        Ok(template::Model {
            // id 是主键，设置为默认值，由数据库生成
            id: 0,

            // 将 DTO 中的字段直接映射到 Model 中
            shop_number: dto.shop_number.unwrap_or(SHOP_NUMBER), //TODO: 需要实现用户登录模块
            name: dto.name,
            source: dto.source,
            target: dto.target,
            goods: dto.goods,
            r#type: dto.r#type,
            stock: dto.stock,

            // 时间字段
            // model.valid_start_time 是 Option<DateTime<Utc>>
            valid_start_time: Some(checked_valid_start_time), // 经过校验，所以这里是 Some(...)
            // model.valid_end_time 是 Option<DateTime<Utc>>
            valid_end_time: Some(checked_valid_end_time),

            // JSON 字符串字段转换为 Option<JsonValue>
            receive_rule: serde_json::from_str(&dto.receive_rule).ok(),
            consume_rule: serde_json::from_str(&dto.consume_rule).ok(),

            // 模型中由系统设置的字段
            status: CouponStatus::Active,
            create_time: Some(Utc::now()),
            update_time: Some(Utc::now()),
            del_flag: 0,
        })
    }
}

// 添加一个从 &TemplateSaveReqDto 到 template::Model 的转换方便使用 (使用 TryFrom)
impl TryFrom<&TemplateSaveReqDto> for template::Model {
    type Error = AppError; // 指定错误类型

    fn try_from(dto: &TemplateSaveReqDto) -> Result<Self, Self::Error> {
        // 调用为 TemplateSaveReqDto 实现的 try_from
        // 这需要克隆 dto，因为 TemplateSaveReqDto 的 try_from 接收所有权
        Self::try_from(dto.clone())
    }
}
