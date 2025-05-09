use serde_repr::{Deserialize_repr, Serialize_repr};

// --- 优惠券来源 ---
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default)]
#[repr(i32)]
pub enum CouponSource {
    #[default]
    Shop = 0,     // 店铺券
    Platform = 1, // 平台券
}

// --- 优惠对象 ---
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default)]
#[repr(i32)]
pub enum CouponTarget {
    #[default]
    SpecificGoods = 0, // 商品专属
    StoreWide = 1,     // 全店通用
}

// --- 优惠类型 ---
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default)]
#[repr(i32)]
pub enum CouponType {
    #[default]
    InstantReduction = 0, // 立减券
    FullReduction = 1,    // 满减券
    Discount = 2,         // 折扣券
}

// --- 优惠券状态 ---
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Default)]
#[repr(i32)]
pub enum CouponStatus {
    #[default]
    Active = 0, // 生效中
    Ended = 1,  // 已结束
}