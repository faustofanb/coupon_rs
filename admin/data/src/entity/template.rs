use crate::enums::{CouponSource, CouponStatus, CouponTarget, CouponType};
use chrono::{DateTime, Utc};
use common::datetime::serde_option_datetime_utc_as_gmt8_string;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait, PrimaryKeyTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// 优惠券模板数据对象
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "t_coupon_template")]
pub struct Model {
    /// 优惠券模板ID，主键
    #[sea_orm(primary_key)]
    pub id: i64,

    /// 商店编号
    pub shop_number: i64,

    /// 优惠券名称
    pub name: String,

    /// 优惠券来源
    pub source: CouponSource,

    /// 优惠对象
    pub target: CouponTarget,

    /// 优惠商品编码
    pub goods: String,

    /// 优惠类型
    #[serde(rename = "type")] // JSON 字段名仍为 "type"
    pub r#type: CouponType,

    /// 有效期开始时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub valid_start_time: Option<DateTime<Utc>>,

    /// 有效期结束时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub valid_end_time: Option<DateTime<Utc>>,

    /// 库存数量
    pub stock: i32,

    /// 领取规则
    #[sea_orm(column_type = "JsonBinary")]
    pub receive_rule: Option<JsonValue>, // 保持 Option 因为 DTO->Model 转换时 .ok() 可能产生 None

    /// 消费规则
    #[sea_orm(column_type = "JsonBinary")]
    pub consume_rule: Option<JsonValue>, // 保持 Option 因为 DTO->Model 转换时 .ok() 可能产生 None

    /// 优惠券状态
    pub status: CouponStatus,

    /// 创建时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub create_time: Option<DateTime<Utc>>,

    /// 更新时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub update_time: Option<DateTime<Utc>>,

    /// 删除标志
    pub del_flag: i32,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// 保持简单的行为定义
impl ActiveModelBehavior for ActiveModel {}
