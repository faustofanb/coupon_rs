use chrono::FixedOffset;

// 公共常量和辅助函数，供本模块内的子模块和外部使用
pub const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const GMT8_OFFSET_SECONDS: i32 = 8 * 3600;

pub fn gmt8_offset() -> FixedOffset {
    FixedOffset::east_opt(GMT8_OFFSET_SECONDS).unwrap()
}

/// 用于 Option<DateTime<Utc>> 和 GMT+8 字符串 "yyyy-MM-dd HH:mm:ss" 之间的序列化/反序列化
pub mod serde_option_datetime_utc_as_gmt8_string {
    use super::{gmt8_offset, FORMAT};
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// 序列化 Option<DateTime<Utc>> 为 GMT+8 时区的 "yyyy-MM-dd HH:mm:ss" 字符串
    pub fn serialize<S>(opt_date_utc: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_date_utc {
            Some(date_utc) => {
                let date_gmt8 = date_utc.with_timezone(&gmt8_offset());
                let s = date_gmt8.format(FORMAT).to_string();
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    /// 从 GMT+8 时区的 "yyyy-MM-dd HH:mm:ss" 字符串反序列化为 Option<DateTime<Utc>>
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_s: Option<String> = Option::deserialize(deserializer)?;
        match opt_s {
            Some(s) => {
                let naive_dt = NaiveDateTime::parse_from_str(&s, FORMAT)
                    .map_err(|e| serde::de::Error::custom(format!("解析日期字符串 '{}' 失败: {}", s, e)))?;
                match gmt8_offset().from_local_datetime(&naive_dt).single() {
                    Some(date_gmt8_fixed) => Ok(Some(date_gmt8_fixed.with_timezone(&Utc))),
                    None => Err(serde::de::Error::custom(format!(
                        "日期时间 '{}' 在 GMT+8 时区无效或有歧义",
                        s
                    ))),
                }
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests_for_utc_formatter {
    use super::gmt8_offset;
    // 针对 serde_option_datetime_utc_as_gmt8_string 的测试
    use super::serde_option_datetime_utc_as_gmt8_string;

    use chrono::{DateTime, NaiveDate, TimeZone, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestUtcContainer {
        #[serde(with = "serde_option_datetime_utc_as_gmt8_string", default)]
        timestamp: Option<DateTime<Utc>>,
    }

    #[test]
    fn serialize_utc_to_gmt8_string() {
        let naive_utc = NaiveDate::from_ymd_opt(2024, 7, 20).unwrap().and_hms_opt(2, 30, 0).unwrap();
        let dt_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_utc, Utc);
        let test_struct = TestUtcContainer { timestamp: Some(dt_utc) };
        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"timestamp":"2024-07-20 10:30:00"}"#); // 2:30 UTC -> 10:30 GMT+8
    }

    #[test]
    fn serialize_none_utc_to_null() {
        let test_struct = TestUtcContainer { timestamp: None };
        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"timestamp":null}"#);
    }

    #[test]
    fn deserialize_gmt8_string_to_utc() {
        let input_gmt8_string = "2024-07-20 15:45:00"; // This is GMT+8 local time
        let json = format!(r#"{{"timestamp":"{}"}}"#, input_gmt8_string);

        let deserialized: TestUtcContainer = serde_json::from_str(&json).unwrap();
        assert!(deserialized.timestamp.is_some());

        let naive_dt_from_string = NaiveDate::from_ymd_opt(2024, 7, 20).unwrap().and_hms_opt(15, 45, 0).unwrap();
        let expected_utc = gmt8_offset()
            .from_local_datetime(&naive_dt_from_string)
            .single()
            .unwrap()
            .with_timezone(&Utc); // 15:45 GMT+8 -> 07:45 UTC

        assert_eq!(deserialized.timestamp, Some(expected_utc));
    }

    #[test]
    fn deserialize_null_to_none_utc() {
        let json = r#"{"timestamp":null}"#;
        let deserialized: TestUtcContainer = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.timestamp, None);
    }

    #[test]
    fn deserialize_missing_field_to_none_utc() {
        let json = r#"{}"#;
        let deserialized: TestUtcContainer = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.timestamp, None);
    }

    #[test]
    fn deserialize_invalid_date_string_for_utc() {
        let json = r#"{"timestamp":"2024-13-01 10:00:00"}"#;
        let result: Result<TestUtcContainer, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}


// /// 用于 Option<DateTime<FixedOffset>> 和 GMT+8 字符串 "yyyy-MM-dd HH:mm:ss" 之间的序列化/反序列化
// pub mod serde_option_datetime_fixed_as_gmt8_string {
//     use super::{gmt8_offset, FORMAT};
//     use chrono::{DateTime, FixedOffset, NaiveDateTime};
//     use serde::{self, Deserialize, Deserializer, Serializer};
//
//     /// 将 `Option<DateTime<FixedOffset>>` 序列化为 "yyyy-MM-dd HH:mm:ss" 格式的字符串，
//     /// 表示 GMT+8 时区的时间。
//     pub fn serialize<S>(date: &Option<DateTime<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match date {
//             Some(dt) => {
//                 let dt_gmt8 = dt.with_timezone(&gmt8_offset());
//                 let s = dt_gmt8.format(FORMAT).to_string();
//                 serializer.serialize_str(&s)
//             }
//             None => serializer.serialize_none(),
//         }
//     }
//
//     /// 将 "yyyy-MM-dd HH:mm:ss" 格式的字符串（假定为 GMT+8 本地时间）
//     /// 反序列化为 `Option<DateTime<FixedOffset>>`。
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let opt_s: Option<String> = Option::deserialize(deserializer)?;
//         match opt_s {
//             Some(s) => {
//                 match NaiveDateTime::parse_from_str(&s, FORMAT) {
//                     Ok(naive_datetime) => {
//                         // 当前的实现：将 naive_datetime 视为 UTC 组件，然后附加 GMT+8 偏移量。
//                         // 这会导致结果的 UTC 部分是 naive_datetime，本地部分是 naive_datetime + 8h。
//                         // 如果意图是 s 本身就是 GMT+8 的本地时间，那么应该用:
//                         // gmt8_offset().from_local_datetime(&naive_datetime).single().ok_or_else(...)
//                         Ok(Some(DateTime::<FixedOffset>::from_naive_utc_and_offset(naive_datetime, gmt8_offset())))
//                     }
//                     Err(e) => Err(serde::de::Error::custom(format!(
//                         "解析日期字符串 '{}' (格式 '{}') 时出错: {}",
//                         s, FORMAT, e
//                     ))),
//                 }
//             }
//             None => Ok(None),
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests_for_fixed_formatter { // 针对 serde_option_datetime_fixed_as_gmt8_string 的测试
//     use super::serde_option_datetime_fixed_as_gmt8_string as serde_datetime_gmt8_format;
//
//     use super::gmt8_offset;
//     // 使用父模块的公共辅助函数
//         use super::FORMAT;
//     // 使用父模块的常量
//
//     use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
//     use serde::{Deserialize, Serialize};
//     use serde_json;
//
//     #[derive(Serialize, Deserialize, Debug, PartialEq)]
//     struct TestFixedOffsetContainer {
//         // 你在之前的测试中使用了 serde_datetime_gmt8_format，我这里保持一致
//         #[serde(with = "serde_datetime_gmt8_format", default)]
//         timestamp: Option<DateTime<FixedOffset>>,
//     }
//
//     // --- 序列化测试 (与你之前的测试类似) ---
//     #[test]
//     fn test_serialize_fixed_already_in_gmt8() {
//         let gmt8 = gmt8_offset();
//         let naive_local_dt = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap().and_hms_opt(10, 30, 0).unwrap();
//         let dt_gmt8 = gmt8.from_local_datetime(&naive_local_dt).single().unwrap();
//         let test_struct = TestFixedOffsetContainer { timestamp: Some(dt_gmt8) };
//         let json = serde_json::to_string(&test_struct).unwrap();
//         assert_eq!(json, r#"{"timestamp":"2024-07-15 10:30:00"}"#);
//     }
//
//     #[test]
//     fn test_serialize_fixed_from_utc_to_gmt8_format() {
//         let naive_utc_dt = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap().and_hms_opt(2, 30, 0).unwrap();
//         let dt_utc = Utc.from_local_datetime(&naive_utc_dt).single().unwrap();
//         let dt_fixed_utc_offset = dt_utc.with_timezone(&FixedOffset::east_opt(0).unwrap());
//         let test_struct = TestFixedOffsetContainer { timestamp: Some(dt_fixed_utc_offset) };
//         let json = serde_json::to_string(&test_struct).unwrap();
//         assert_eq!(json, r#"{"timestamp":"2024-07-15 10:30:00"}"#);
//     }
//
//     #[test]
//     fn test_serialize_none_fixed_to_null() {
//         let test_struct = TestFixedOffsetContainer { timestamp: None };
//         let json = serde_json::to_string(&test_struct).unwrap();
//         assert_eq!(json, r#"{"timestamp":null}"#);
//     }
//
//     // --- 反序列化测试 (与你之前的测试类似, 调整以反映 FixedOffset 的行为) ---
//     #[test]
//     fn test_deserialize_valid_gmt8_string_to_fixed_offset_current_logic() {
//         let input_date_string = "2024-07-15 14:45:30";
//         let json = format!(r#"{{"timestamp":"{}"}}"#, input_date_string);
//         let deserialized: TestFixedOffsetContainer = serde_json::from_str(&json).unwrap();
//         assert!(deserialized.timestamp.is_some());
//         let parsed_dt = deserialized.timestamp.unwrap();
//
//         let naive_from_string = NaiveDateTime::parse_from_str(input_date_string, FORMAT).unwrap();
//         // 当前 fixed_offset formatter 的行为: naive_from_string 是 UTC 部分
//         let expected_dt_utc_equivalent = DateTime::<Utc>::from_naive_utc_and_offset(naive_from_string, Utc);
//
//         assert_eq!(parsed_dt.with_timezone(&Utc), expected_dt_utc_equivalent, "UTC 等价值不匹配");
//         assert_eq!(parsed_dt.offset(), &gmt8_offset(), "偏移量不匹配");
//         let expected_local_representation_in_gmt8 = naive_from_string + Duration::seconds(super::GMT8_OFFSET_SECONDS as i64);
//         assert_eq!(parsed_dt.naive_local(), expected_local_representation_in_gmt8, "GMT+8 本地表示不匹配");
//     }
//
//     #[test]
//     fn test_deserialize_null_to_none_fixed() {
//         let json = r#"{"timestamp":null}"#;
//         let deserialized: TestFixedOffsetContainer = serde_json::from_str(&json).unwrap();
//         assert_eq!(deserialized.timestamp, None);
//     }
//
//     #[test]
//     fn test_deserialize_missing_field_to_none_fixed() {
//         let json = r#"{}"#;
//         let deserialized: TestFixedOffsetContainer = serde_json::from_str(&json).unwrap();
//         assert_eq!(deserialized.timestamp, None);
//     }
//
//     #[test]
//     fn test_deserialize_invalid_date_string_for_fixed() {
//         let json = r#"{"timestamp":"2024-02-30 10:00:00"}"#;
//         let result: Result<TestFixedOffsetContainer, _> = serde_json::from_str(json);
//         assert!(result.is_err());
//     }
// }