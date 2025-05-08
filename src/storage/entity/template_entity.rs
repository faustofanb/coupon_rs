use crate::common::enums::{CouponSource, CouponStatus, CouponTarget, CouponType};
use crate::util::datetime::serde_option_datetime_utc_as_gmt8_string;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{Error as SqlxError, FromRow, Row};

/// 优惠券模板数据对象
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TemplateDO {
    /// 优惠券模板ID，主键
    pub id: Option<u64>,
    /// 商店编号
    pub shop_number: Option<u64>,
    /// 优惠券名称
    pub name: Option<String>,

    /// 优惠券来源
    // serde_repr 会处理 Option<Enum> 与 Option<i32> 之间的 JSON 转换
    pub source: Option<CouponSource>,

    /// 优惠对象
    pub target: Option<CouponTarget>,

    /// 优惠商品编码
    pub goods: Option<String>,

    /// 优惠类型
    #[serde(rename = "type")] // JSON 字段名仍为 "type"
    pub r#type: Option<CouponType>,

    /// 有效期开始时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub valid_start_time: Option<DateTime<Utc>>,

    /// 有效期结束时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub valid_end_time: Option<DateTime<Utc>>,

    /// 库存数量
    pub stock: Option<i32>,
    /// 领取规则
    pub receive_rule: Option<String>,
    /// 消费规则
    pub consume_rule: Option<String>,

    /// 优惠券状态
    pub status: Option<CouponStatus>,

    /// 创建时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub create_time: Option<DateTime<Utc>>,

    /// 更新时间 (JSON 中为 GMT+8 字符串, Rust 内部为 UTC)
    #[serde(with = "serde_option_datetime_utc_as_gmt8_string")]
    pub update_time: Option<DateTime<Utc>>,

    /// 删除标志
    pub del_flag: Option<i32>, // del_flag 保持 Option<i32>，或也可以转为 Option<bool> 并自定义处理
}

impl<'r> FromRow<'r, MySqlRow> for TemplateDO {
    fn from_row(row: &'r MySqlRow) -> Result<Self, SqlxError> {
        let to_option_utc = |opt_naive_dt: Option<NaiveDateTime>| {
            opt_naive_dt.map(|naive_dt| Utc.from_utc_datetime(&naive_dt))
        };

        Ok(TemplateDO {
            id: row.try_get("id")?,
            shop_number: row.try_get("shop_number")?,
            name: row.try_get("name")?,

            // sqlx 会使用 #[sqlx(Type)] 和 #[repr(i32)] 来映射
            // 如果数据库列是 NULL， try_get 会返回 Ok(None)
            source: row.try_get("source")?,
            target: row.try_get("target")?,
            goods: row.try_get("goods")?,
            r#type: row.try_get("type")?,

            valid_start_time: row.try_get::<Option<NaiveDateTime>, _>("valid_start_time").map(to_option_utc)?,
            valid_end_time: row.try_get::<Option<NaiveDateTime>, _>("valid_end_time").map(to_option_utc)?,
            stock: row.try_get("stock")?,
            receive_rule: row.try_get("receive_rule")?,
            consume_rule: row.try_get("consume_rule")?,
            status: row.try_get("status")?,
            create_time: row.try_get::<Option<NaiveDateTime>, _>("create_time").map(to_option_utc)?,
            update_time: row.try_get::<Option<NaiveDateTime>, _>("update_time").map(to_option_utc)?,
            del_flag: row.try_get("del_flag")?,
        })
    }
}

// --- 单元测试部分 ---
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::enums::{CouponSource, CouponStatus, CouponTarget, CouponType};
    use chrono::{Duration, NaiveDate, TimeZone, Utc};
    use serde_json;
    use crate::util::datetime::{gmt8_offset, FORMAT};
    // --- JSON 序列化/反序列化测试 ---

    #[test]
    fn test_template_do_json_serialization_deserialization() {
        // 构造一个 UTC 时间点用于测试
        let test_start_utc = Utc.with_ymd_and_hms(2024, 7, 22, 10, 0, 0).unwrap();
        let test_end_utc = test_start_utc + Duration::days(15);
        let test_create_utc = test_start_utc - Duration::hours(2); // 2024-07-22 08:00:00 UTC

        let original_do = TemplateDO {
            id: Some(101),
            shop_number: Some(202),
            name: Some("JSON测试优惠券".to_string()),
            source: Some(CouponSource::Platform),   // 1
            target: Some(CouponTarget::StoreWide), // 1
            goods: None,
            r#type: Some(CouponType::Discount),     // 2
            valid_start_time: Some(test_start_utc),
            valid_end_time: Some(test_end_utc),
            stock: Some(1000),
            receive_rule: Some("{\"limit\":1}".to_string()),
            consume_rule: Some("{\"min_spend\":100}".to_string()),
            status: Some(CouponStatus::Active),     // 0
            create_time: Some(test_create_utc),
            update_time: None,
            del_flag: Some(0),
        };

        // 序列化
        let json_string = serde_json::to_string_pretty(&original_do).unwrap();
        println!("序列化后的 TemplateDO (JSON):\n{}", json_string);

        // 验证序列化结果 - 使用 camelCase
        assert!(json_string.contains(&format!("\"id\": {}", original_do.id.unwrap())));
        assert!(json_string.contains(&format!("\"shopNumber\": {}", original_do.shop_number.unwrap())));
        assert!(json_string.contains("\"name\": \"JSON测试优惠券\""));
        assert!(json_string.contains(&format!("\"source\": {}", CouponSource::Platform as i32))); // 1
        assert!(json_string.contains(&format!("\"target\": {}", CouponTarget::StoreWide as i32))); // 1
        assert!(json_string.contains("\"goods\": null"));
        assert!(json_string.contains(&format!("\"type\": {}", CouponType::Discount as i32))); // 2, key 是 "type"
        assert!(json_string.contains(&format!("\"status\": {}", CouponStatus::Active as i32))); // 0

        // 验证日期时间序列化为 GMT+8 字符串
        let expected_start_gmt8_str = test_start_utc.with_timezone(&gmt8_offset()).format(FORMAT).to_string();
        assert!(json_string.contains(&format!("\"validStartTime\": \"{}\"", expected_start_gmt8_str))); // camelCase key

        let expected_end_gmt8_str = test_end_utc.with_timezone(&gmt8_offset()).format(FORMAT).to_string();
        assert!(json_string.contains(&format!("\"validEndTime\": \"{}\"", expected_end_gmt8_str)));   // camelCase key

        let expected_create_gmt8_str = test_create_utc.with_timezone(&gmt8_offset()).format(FORMAT).to_string();
        assert!(json_string.contains(&format!("\"createTime\": \"{}\"", expected_create_gmt8_str))); // camelCase key

        assert!(json_string.contains("\"updateTime\": null")); // camelCase key
        assert!(json_string.contains(&format!("\"delFlag\": {}", original_do.del_flag.unwrap()))); // camelCase key

        // 反序列化
        let deserialized_do: TemplateDO = serde_json::from_str(&json_string).unwrap();
        assert_eq!(original_do, deserialized_do);

        // 单独验证反序列化后的一些关键字段
        assert_eq!(deserialized_do.r#type, Some(CouponType::Discount));
        assert_eq!(deserialized_do.valid_start_time, Some(test_start_utc)); // 确保内部是 UTC
        assert_eq!(deserialized_do.update_time, None);


        // 测试从一个典型的 JSON payload 反序列化
        let input_json = r#"{
            "id": 102,
            "shopNumber": 203,
            "name": "来自JSON的优惠",
            "source": 0,
            "target": 0,
            "goods": "特定商品ABC",
            "type": 1,
            "validStartTime": "2024-08-01 09:30:00",
            "validEndTime": "2024-08-15 23:59:59",
            "stock": 50,
            "receiveRule": "{}",
            "consumeRule": "{}",
            "status": 1,
            "createTime": "2024-07-31 12:00:00",
            "updateTime": null,
            "delFlag": 0
        }"#;

        let parsed_do: TemplateDO = serde_json::from_str(input_json).unwrap();
        assert_eq!(parsed_do.id, Some(102));
        assert_eq!(parsed_do.name, Some("来自JSON的优惠".to_string()));
        assert_eq!(parsed_do.source, Some(CouponSource::Shop));
        assert_eq!(parsed_do.target, Some(CouponTarget::SpecificGoods));
        assert_eq!(parsed_do.goods, Some("特定商品ABC".to_string()));
        assert_eq!(parsed_do.r#type, Some(CouponType::FullReduction));
        assert_eq!(parsed_do.status, Some(CouponStatus::Ended));
        assert_eq!(parsed_do.update_time, None);

        // 验证日期时间转换: "2024-08-01 09:30:00" (GMT+8) -> UTC
        let expected_start_utc = gmt8_offset().from_local_datetime(
            &NaiveDate::from_ymd_opt(2024, 8, 1).unwrap().and_hms_opt(9, 30, 0).unwrap()
        ).single().unwrap().with_timezone(&Utc);
        assert_eq!(parsed_do.valid_start_time, Some(expected_start_utc));
    }

}