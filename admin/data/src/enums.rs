use sea_orm::{DeriveActiveEnum, EnumIter};
use serde_repr::{Deserialize_repr, Serialize_repr};

// --- 优惠券来源 ---
#[derive(
    Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum CouponSource {
    #[default]
    #[sea_orm(num_value = 0)]
    Shop = 0, // 店铺券

    #[sea_orm(num_value = 1)]
    Platform = 1, // 平台券
}

// --- 优惠对象 ---
#[derive(
    Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum CouponTarget {
    #[default]
    #[sea_orm(num_value = 0)]
    SpecificGoods = 0, // 商品专属

    #[sea_orm(num_value = 1)]
    StoreWide = 1, // 全店通用
}

// --- 优惠类型 ---
#[derive(
    Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum CouponType {
    #[default]
    #[sea_orm(num_value = 0)]
    InstantReduction = 0, // 立减券

    #[sea_orm(num_value = 1)]
    FullReduction = 1, // 满减券

    #[sea_orm(num_value = 2)]
    Discount = 2, // 折扣券
}

// --- 优惠券状态 ---
#[derive(
    Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum CouponStatus {
    #[default]
    #[sea_orm(num_value = 0)]
    Active = 0, // 生效中

    #[sea_orm(num_value = 1)]
    Ended = 1, // 已结束
}
