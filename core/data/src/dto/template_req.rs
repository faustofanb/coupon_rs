use common::datetime::serde_option_datetime_utc_as_gmt8_string;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use common::enums::{CouponSource, CouponTarget, CouponType};

/// 优惠券模板新增/保存请求 DTO
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TemplateSaveReqDto {
    /// 优惠券名称
    /// 示例: "用户下单满10减3特大优惠"
    pub name: Option<String>,

    /// 优惠券来源
    /// 示例: 0 (店铺券)
    pub source: Option<CouponSource>, // 使用枚举

    /// 优惠对象
    /// 示例: 1 (全店通用)
    pub target: Option<CouponTarget>, // 使用枚举

    /// 优惠商品编码 (如果 target 是商品专属)
    pub goods: Option<String>,

    /// 优惠类型
    /// 示例: 0 (立减券)
    #[serde(rename = "type")]
    pub r#type: Option<CouponType>, // 使用枚举

    /// 有效期开始时间
    /// JSON 示例: "2024-07-08 12:00:00" (将被解释为 GMT+8 并转换为 UTC 存储)
    #[serde(
        with = "serde_option_datetime_utc_as_gmt8_string",
        default // 确保字段缺失时，Option<DateTime<Utc>> 默认为 None
    )]
    pub valid_start_time: Option<DateTime<Utc>>, // 内部存储为 UTC

    /// 有效期结束时间
    /// JSON 示例: "2025-07-08 12:00:00" (将被解释为 GMT+8 并转换为 UTC 存储)
    #[serde(
        with = "serde_option_datetime_utc_as_gmt8_string",
        default
    )]
    pub valid_end_time: Option<DateTime<Utc>>, // 内部存储为 UTC

    /// 库存
    /// 示例: "200"
    pub stock: Option<i32>,

    /// 领取规则 (JSON 字符串)
    /// 示例: "{\"limitPerPerson\":1,\"usageInstructions\":\"3\"}"
    pub receive_rule: Option<String>,

    /// 消耗规则 (JSON 字符串)
    /// 示例: "{\"termsOfUse\":10,\"maximumDiscountAmount\":3,...}"
    pub consume_rule: Option<String>,
}

// --- 单元测试部分 ---
#[cfg(test)]
mod tests {
    use super::*; // 导入 TemplateSaveReqDto 和上面 use 的枚举、serde 模块
    // use crate::util::datetime::{gmt8_offset, FORMAT}; // 如果测试需要直接访问这些
    use chrono::{Duration, FixedOffset, NaiveDate, TimeZone, Utc};
    use serde_json;

    // 辅助函数获取 GMT+8 偏移量，用于构造测试中的预期值
    fn gmt8_fixed_offset() -> FixedOffset {
        FixedOffset::east_opt(8 * 3600).unwrap()
    }

    #[test]
    fn test_template_save_req_dto_serialization_deserialization() {
        // 构造一个 UTC 时间点用于测试
        // 例如：2024-07-21 08:00:00 UTC
        // 这个时间在 GMT+8 是 2024-07-21 16:00:00
        let test_start_utc = Utc.from_local_datetime(
            &NaiveDate::from_ymd_opt(2024, 7, 21).unwrap().and_hms_opt(8, 0, 0).unwrap()
        ).single().unwrap();

        let test_end_utc = test_start_utc + Duration::days(30);

        let dto = TemplateSaveReqDto {
            name: Some("年中大促".to_string()),
            source: Some(CouponSource::Platform), // 平台券 (对应数字 1)
            target: Some(CouponTarget::StoreWide), // 全店通用 (对应数字 1)
            goods: None,
            r#type: Some(CouponType::FullReduction), // 满减券 (对应数字 1)
            valid_start_time: Some(test_start_utc),
            valid_end_time: Some(test_end_utc),
            stock: Some(1000),
            receive_rule: Some("{}".to_string()),
            consume_rule: Some("{}".to_string()),
        };

        // 序列化
        let json_string = serde_json::to_string_pretty(&dto).unwrap();
        println!("Serialized DTO (UTC internal, GMT+8 string): {}", json_string);

        // 预期：
        // source, target, type 会被 serde_repr 序列化为它们底层的 i32 值
        // valid_start_time (2024-07-21 08:00:00 UTC) 会被 serde_option_datetime_utc_as_gmt8_string
        // 序列化为 GMT+8 字符串 "2024-07-21 16:00:00"
        assert!(json_string.contains("\"name\": \"年中大促\""));
        assert!(json_string.contains("\"source\": 1"));
        assert!(json_string.contains("\"target\": 1"));
        assert!(json_string.contains("\"type\": 1"));
        assert!(json_string.contains("\"validStartTime\": \"2024-07-21 16:00:00\""));
        let expected_end_gmt8_str = test_end_utc.with_timezone(&gmt8_fixed_offset()).format("%Y-%m-%d %H:%M:%S").to_string();
        assert!(json_string.contains(&format!("\"validEndTime\": \"{}\"", expected_end_gmt8_str)));

        // 反序列化
        let deserialized_dto: TemplateSaveReqDto = serde_json::from_str(&json_string).unwrap();
        assert_eq!(dto, deserialized_dto); // 应该完全相等

        // 单独验证反序列化后的枚举和时间
        assert_eq!(deserialized_dto.source, Some(CouponSource::Platform));
        assert_eq!(deserialized_dto.valid_start_time, Some(test_start_utc));


        // 测试从包含 GMT+8 字符串和数字枚举的 JSON 反序列化
        let input_json = r#"{
            "name": "来自JSON的优惠",
            "source": 0,
            "target": 0,
            "type": 2,
            "validStartTime": "2024-08-01 10:00:00",
            "validEndTime": "2024-08-31 23:59:59",
            "stock": 50
        }"#; // 其他字段缺失，会是 None

        let parsed_dto: TemplateSaveReqDto = serde_json::from_str(input_json).unwrap();

        assert_eq!(parsed_dto.name, Some("来自JSON的优惠".to_string()));
        assert_eq!(parsed_dto.source, Some(CouponSource::Shop)); // 0 -> Shop
        assert_eq!(parsed_dto.target, Some(CouponTarget::SpecificGoods)); // 0 -> SpecificGoods
        assert_eq!(parsed_dto.r#type, Some(CouponType::Discount)); // 2 -> Discount
        assert_eq!(parsed_dto.stock, Some(50));
        assert_eq!(parsed_dto.goods, None); // 缺失

        // 验证 "2024-08-01 10:00:00" (GMT+8) 是否正确转换为 UTC
        let expected_start_utc_from_json = gmt8_fixed_offset()
            .from_local_datetime(&NaiveDate::from_ymd_opt(2024, 8, 1).unwrap().and_hms_opt(10, 0, 0).unwrap())
            .single().unwrap()
            .with_timezone(&Utc);
        assert_eq!(parsed_dto.valid_start_time, Some(expected_start_utc_from_json));

        let expected_end_utc_from_json = gmt8_fixed_offset()
            .from_local_datetime(&NaiveDate::from_ymd_opt(2024, 8, 31).unwrap().and_hms_opt(23, 59, 59).unwrap())
            .single().unwrap()
            .with_timezone(&Utc);
        assert_eq!(parsed_dto.valid_end_time, Some(expected_end_utc_from_json));
    }
}