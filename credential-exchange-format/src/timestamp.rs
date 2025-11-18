//! # Flexible Timestamp Serialization
//!
//! This module provides custom serde serialization and deserialization functions for
//! [`DateTime<Utc>`] that support multiple input formats while maintaining consistent output.
//!
//! ## Deserialization
//!
//! The deserializers accept timestamps in two formats:
//! - **UNIX timestamps**: Integer values (i64 or u64) representing seconds since the Unix epoch
//! - **ISO8601 strings**: RFC3339-compliant datetime strings (e.g., `"2023-11-18T10:30:00Z"`)
//!
//! ## Serialization
//!
//! All timestamps are serialized as UNIX timestamps (i64) for consistency and compatibility
//! with the CXF standard.
//!
//! ## Usage
//!
//! Use the `#[serde(with = "timestamp")]` attribute for required timestamp fields:
//!
//! ```rust
//! use chrono::{DateTime, Utc};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Event {
//!     #[serde(with = "credential_exchange_format::timestamp")]
//!     timestamp: DateTime<Utc>,
//! }
//! ```
//!
//! For optional timestamp fields, use `#[serde(with = "timestamp::option")]`:
//!
//! ```rust
//! use chrono::{DateTime, Utc};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Event {
//!     #[serde(with = "credential_exchange_format::timestamp::option")]
//!     created_at: Option<DateTime<Utc>>,
//! }
//! ```

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serializer};

/// Serializes a [`DateTime<Utc>`] as a UNIX timestamp (i64).
///
/// This function is intended to be used with serde's `#[serde(with = "...")]` attribute.
///
/// # Errors
///
/// Returns an error if the serializer fails to serialize the timestamp value.
///
/// # Examples
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Event {
///     #[serde(with = "credential_exchange_format::timestamp")]
///     timestamp: DateTime<Utc>,
/// }
/// ```
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.timestamp())
}

/// Deserializes a [`DateTime<Utc>`] from either a UNIX timestamp (i64/u64) or an ISO8601 string.
///
/// This function is intended to be used with serde's `#[serde(with = "...")]` attribute.
///
/// # Accepted Formats
///
/// - UNIX timestamp as i64 or u64 (seconds since Unix epoch)
/// - ISO8601/RFC3339 string (e.g., `"2023-11-18T10:30:00Z"`)
///
/// # Errors
///
/// Returns an error if:
/// - The timestamp value is invalid or out of range
/// - The ISO8601 string cannot be parsed
/// - The input is neither a number nor a string
///
/// # Examples
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Event {
///     #[serde(with = "credential_exchange_format::timestamp")]
///     timestamp: DateTime<Utc>,
/// }
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TimestampVisitor;

    impl serde::de::Visitor<'_> for TimestampVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a UNIX timestamp (u64) or ISO8601 string")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Utc.timestamp_opt(value, 0)
                .single()
                .ok_or_else(|| E::custom(format!("invalid timestamp: {value}")))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            #[allow(clippy::cast_possible_wrap)]
            self.visit_i64(value as i64)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value
                .parse::<DateTime<Utc>>()
                .map_err(|e| E::custom(format!("invalid ISO8601: {e}")))
        }
    }

    deserializer.deserialize_any(TimestampVisitor)
}

/// Serialization and deserialization functions for `Option<DateTime<Utc>>`.
///
/// This module provides the same flexible deserialization as the parent module,
/// but for optional timestamp fields.
///
/// # Usage
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Event {
///     #[serde(with = "credential_exchange_format::timestamp::option")]
///     created_at: Option<DateTime<Utc>>,
/// }
/// ```
pub mod option {
    use super::{DateTime, Deserialize, Deserializer, Serializer, TimeZone, Utc};

    /// Serializes an `Option<DateTime<Utc>>` as either a UNIX timestamp (i64) or null.
    ///
    /// This function is intended to be used with serde's `#[serde(with = "...")]` attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the serializer fails to serialize the timestamp value.
    #[allow(clippy::ref_option)]
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_some(&dt.timestamp()),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes an `Option<DateTime<Utc>>` from either a UNIX timestamp, an ISO8601 string,
    /// or null.
    ///
    /// This function is intended to be used with serde's `#[serde(with = "...")]` attribute.
    ///
    /// # Accepted Formats
    ///
    /// - UNIX timestamp as i64 or u64 (seconds since Unix epoch)
    /// - ISO8601/RFC3339 string (e.g., `"2023-11-18T10:30:00Z"`)
    /// - null
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The timestamp value is invalid or out of range
    /// - The ISO8601 string cannot be parsed
    /// - The input is neither a number, string, nor null
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<serde_json::Value>::deserialize(deserializer)?
            .map(|v| {
                v.as_i64().map_or_else(
                    || {
                        v.as_str().map_or_else(
                            || Err(serde::de::Error::custom("expected number or string")),
                            |s| {
                                s.parse::<DateTime<Utc>>().map_err(|e| {
                                    serde::de::Error::custom(format!("invalid ISO8601: {e}"))
                                })
                            },
                        )
                    },
                    |num| {
                        Utc.timestamp_opt(num, 0)
                            .single()
                            .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
                    },
                )
            })
            .transpose()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::unreadable_literal)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(with = "crate::timestamp")]
        timestamp: DateTime<Utc>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStructOptional {
        #[serde(with = "crate::timestamp::option")]
        timestamp: Option<DateTime<Utc>>,
    }

    #[test]
    fn test_deserialize_from_unix_timestamp() {
        let json = r#"{"timestamp": 1700000000}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        let expected = Utc.timestamp_opt(1700000000, 0).unwrap();
        assert_eq!(result.timestamp, expected);
    }

    #[test]
    fn test_deserialize_from_iso8601_string() {
        let json = r#"{"timestamp": "2023-11-14T22:13:20Z"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        let expected = Utc.with_ymd_and_hms(2023, 11, 14, 22, 13, 20).unwrap();
        assert_eq!(result.timestamp, expected);
    }

    #[test]
    fn test_deserialize_from_iso8601_with_offset() {
        let json = r#"{"timestamp": "2023-11-14T22:13:20+00:00"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        let expected = Utc.with_ymd_and_hms(2023, 11, 14, 22, 13, 20).unwrap();
        assert_eq!(result.timestamp, expected);
    }

    #[test]
    fn test_serialize_to_unix_timestamp() {
        let timestamp = Utc.timestamp_opt(1700000000, 0).unwrap();
        let test_struct = TestStruct { timestamp };
        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"timestamp":1700000000}"#);
    }

    #[test]
    fn test_roundtrip_unix_timestamp() {
        let original_json = r#"{"timestamp": 1700000000}"#;
        let parsed: TestStruct = serde_json::from_str(original_json).unwrap();
        let serialized = serde_json::to_string(&parsed).unwrap();
        assert_eq!(serialized, r#"{"timestamp":1700000000}"#);
    }

    #[test]
    fn test_roundtrip_iso8601_to_unix() {
        // ISO8601 input should serialize to UNIX timestamp
        let original_json = r#"{"timestamp": "2023-11-14T22:13:20Z"}"#;
        let parsed: TestStruct = serde_json::from_str(original_json).unwrap();
        let serialized = serde_json::to_string(&parsed).unwrap();
        assert_eq!(serialized, r#"{"timestamp":1700000000}"#);
    }

    #[test]
    fn test_deserialize_invalid_timestamp() {
        let json = r#"{"timestamp": "not a timestamp"}"#;
        let result: Result<TestStruct, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_type() {
        let json = r#"{"timestamp": true}"#;
        let result: Result<TestStruct, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // Tests for Option<DateTime<Utc>>

    #[test]
    fn test_deserialize_optional_from_unix_timestamp() {
        let json = r#"{"timestamp": 1700000000}"#;
        let result: TestStructOptional = serde_json::from_str(json).unwrap();
        let expected = Utc.timestamp_opt(1700000000, 0).unwrap();
        assert_eq!(result.timestamp, Some(expected));
    }

    #[test]
    fn test_deserialize_optional_from_iso8601() {
        let json = r#"{"timestamp": "2023-11-14T22:13:20Z"}"#;
        let result: TestStructOptional = serde_json::from_str(json).unwrap();
        let expected = Utc.with_ymd_and_hms(2023, 11, 14, 22, 13, 20).unwrap();
        assert_eq!(result.timestamp, Some(expected));
    }

    #[test]
    fn test_deserialize_optional_null() {
        let json = r#"{"timestamp": null}"#;
        let result: TestStructOptional = serde_json::from_str(json).unwrap();
        assert_eq!(result.timestamp, None);
    }

    #[test]
    fn test_serialize_optional_some() {
        let timestamp = Utc.timestamp_opt(1700000000, 0).unwrap();
        let test_struct = TestStructOptional {
            timestamp: Some(timestamp),
        };
        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"timestamp":1700000000}"#);
    }

    #[test]
    fn test_serialize_optional_none() {
        let test_struct = TestStructOptional { timestamp: None };
        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"timestamp":null}"#);
    }

    #[test]
    fn test_roundtrip_optional_unix_timestamp() {
        let original_json = r#"{"timestamp": 1700000000}"#;
        let parsed: TestStructOptional = serde_json::from_str(original_json).unwrap();
        let serialized = serde_json::to_string(&parsed).unwrap();
        assert_eq!(serialized, r#"{"timestamp":1700000000}"#);
    }

    #[test]
    fn test_roundtrip_optional_iso8601_to_unix() {
        let original_json = r#"{"timestamp": "2023-11-14T22:13:20Z"}"#;
        let parsed: TestStructOptional = serde_json::from_str(original_json).unwrap();
        let serialized = serde_json::to_string(&parsed).unwrap();
        assert_eq!(serialized, r#"{"timestamp":1700000000}"#);
    }

    #[test]
    fn test_roundtrip_optional_null() {
        let original_json = r#"{"timestamp": null}"#;
        let parsed: TestStructOptional = serde_json::from_str(original_json).unwrap();
        let serialized = serde_json::to_string(&parsed).unwrap();
        assert_eq!(serialized, r#"{"timestamp":null}"#);
    }

    #[test]
    fn test_deserialize_optional_invalid_timestamp() {
        let json = r#"{"timestamp": "not a timestamp"}"#;
        let result: Result<TestStructOptional, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_optional_invalid_type() {
        let json = r#"{"timestamp": true}"#;
        let result: Result<TestStructOptional, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
