use std::{borrow::Cow, fmt, str};

use chrono::{Month, NaiveDate};
use serde::{
    de::{
        value::{StrDeserializer, StringDeserializer},
        DeserializeOwned, Visitor,
    },
    ser::SerializeStruct,
    Deserialize, Serialize,
};

use crate::{B64Url, Extension};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditableField<T, E = ()> {
    /// A unique identifier for the [EditableField] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: Option<B64Url>,
    /// This member contains the fieldType defined by the user.
    pub value: Expected<T>,
    /// This member contains a user facing value describing the value stored. This value MAY be
    /// user defined.
    pub label: Option<String>,
    /// This member permits the exporting provider to add additional information associated to this
    /// [EditableField]. This MAY be used to provide an exchange where a minimal amount of
    /// information is lost.
    pub extensions: Option<Vec<Extension<E>>>,
}

/// A field of the incorrect type that was passed instead of an expected field type.
///
/// For example if the spec requires a `string` type, but the user passes in
/// `concealed-string` instead.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnexpectedField {
    String(EditableFieldString),
    ConcealedString(EditableFieldConcealedString),
    Boolean(EditableFieldBoolean),
    Date(EditableFieldDate),
    YearMonth(EditableFieldYearMonth),
    SubdivisionCode(EditableFieldSubdivisionCode),
    CountryCode(EditableFieldCountryCode),
    WifiNetworkSecurityType(EditableFieldWifiNetworkSecurityType),
    Email(EditableFieldEmail),
    Number(EditableFieldNumber),
    Unknown {
        /// The unexpected field type.
        field_type: FieldType,
        /// The Unknown string passed as a value of type `field_type`.
        ///
        /// Example: a `date` that doesn't conform to `yyyy-mm-dd`.
        value: String,
    },
}

impl UnexpectedField {
    /// Returns the [`FieldType`] for this value.
    #[inline]
    pub fn field_type(&self) -> FieldType {
        match self {
            Self::String(_) => EditableFieldString::field_type(),
            Self::ConcealedString(_) => EditableFieldConcealedString::field_type(),
            Self::Boolean(_) => EditableFieldBoolean::field_type(),
            Self::Date(_) => EditableFieldDate::field_type(),
            Self::YearMonth(_) => EditableFieldYearMonth::field_type(),
            Self::SubdivisionCode(_) => EditableFieldSubdivisionCode::field_type(),
            Self::CountryCode(_) => EditableFieldCountryCode::field_type(),
            Self::WifiNetworkSecurityType(_) => EditableFieldWifiNetworkSecurityType::field_type(),
            Self::Email(_) => EditableFieldEmail::field_type(),
            Self::Number(_) => EditableFieldNumber::field_type(),
            Self::Unknown { field_type, .. } => field_type.clone(),
        }
    }
}

impl From<UnexpectedField> for String {
    #[inline]
    fn from(value: UnexpectedField) -> Self {
        match value {
            UnexpectedField::String(v) => v.0,
            UnexpectedField::ConcealedString(v) => v.0,
            UnexpectedField::WifiNetworkSecurityType(v) => v.into(),
            UnexpectedField::SubdivisionCode(v) => v.0,
            UnexpectedField::CountryCode(v) => v.0,
            UnexpectedField::Unknown { value: v, .. } => v,
            UnexpectedField::Email(v) => v.0,
            UnexpectedField::Number(v) => v.0,
            UnexpectedField::Boolean(v) => v.into(),
            UnexpectedField::Date(v) => v.into(),
            UnexpectedField::YearMonth(v) => v.into(),
        }
    }
}

/// Holds onto an editable field, and records whether it was an expected or unexpected field.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ExpectedInner<T> {
    /// The field we found had the same field type we expected.
    Expected(T),
    /// The field we found had a different field type than what we expected.
    Unexpected(UnexpectedField),
}

/// Holds onto an editable field, and records whether it was an expected or unexpected field.
///
/// This is used as a type-safe wrapper around an editable field to encode expectations around
/// the type of field we are expecting in a credential.
///
/// This can only be instantiated via the exposed `From` implementation so that newly
/// constructed credentials remain spec-compliant with regards to their fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expected<T>(ExpectedInner<T>);

impl<T> Expected<T>
where
    T: EditableFieldType,
{
    /// Returns the real [`FieldType`] of this field.
    #[inline]
    pub fn field_type(&self) -> FieldType {
        match &self.0 {
            ExpectedInner::Expected(_) => T::field_type(),
            ExpectedInner::Unexpected(f) => f.field_type(),
        }
    }

    /// Tries to return the field type we were expecting.
    #[inline]
    pub fn as_expected(&self) -> Result<&T, &UnexpectedField> {
        match &self.0 {
            ExpectedInner::Expected(t) => Ok(t),
            ExpectedInner::Unexpected(f) => Err(f),
        }
    }

    /// Tries to return the field type we were expecting as an owned type.
    #[inline]
    pub fn into_expected(self) -> Result<T, UnexpectedField> {
        match self.0 {
            ExpectedInner::Expected(t) => Ok(t),
            ExpectedInner::Unexpected(f) => Err(f),
        }
    }
}

impl<T> From<T> for Expected<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self(ExpectedInner::Expected(t))
    }
}

impl<T> From<Expected<T>> for String
where
    T: Into<String>,
{
    #[inline]
    fn from(value: Expected<T>) -> Self {
        match value.0 {
            ExpectedInner::Expected(t) => t.into(),
            ExpectedInner::Unexpected(f) => f.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum FieldType {
    /// A UTF-8 encoded string value which is unconcealed and does not have a specified format.
    String,
    /// A UTF-8 encoded string value which should be considered secret and not displayed unless the
    /// user explicitly requests it.
    ConcealedString,
    /// A UTF-8 encoded string value which follows the format specified in
    /// [RFC5322](https://www.rfc-editor.org/rfc/rfc5322#section-3.4). This field SHOULD be
    /// unconcealed.
    Email,
    /// A stringified numeric value which is unconcealed.
    Number,
    /// A boolean value which is unconcealed. It MUST be of the values "true" or "false".
    Boolean,
    /// A string value representing a calendar date which follows the format specified in
    /// [RFC3339](https://www.rfc-editor.org/rfc/rfc3339).
    Date,
    /// A string value representing a calendar date which follows the date-fullyear "-" date-month
    /// pattern as established in [RFC3339](https://www.rfc-editor.org/rfc/rfc3339) Appendix A.
    /// This is equivalent to the `YYYY-MM` format specified in ISO8601.
    YearMonth,
    /// A string value representing a value that SHOULD be a member of WIFINetworkSecurityType.
    WifiNetworkSecurityType,
    /// A string value which MUST follow the ISO3166-1 alpha-2 format.
    SubdivisionCode,
    /// A string which MUST follow the ISO3166-2 format.
    CountryCode,
    #[serde(untagged)]
    Unknown(String),
}

/// A trait to associate the field structs with their `field_type` tag.
pub trait EditableFieldType {
    /// The `field_type` value associated with the type
    fn field_type() -> FieldType;
}

impl<T, E> Serialize for EditableField<T, E>
where
    T: EditableFieldType + Serialize,
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 2
            + self.id.is_some() as usize
            + self.label.is_some() as usize
            + self.extensions.is_some() as usize;
        let mut state = serializer.serialize_struct("editable_field", len)?;

        if let Some(ref id) = self.id {
            state.serialize_field("id", id)?;
        } else {
            state.skip_field("id")?;
        }

        state.serialize_field("fieldType", &self.value.field_type())?;
        match &self.value.0 {
            ExpectedInner::Expected(t) => {
                state.serialize_field("value", &t)?;
            }
            ExpectedInner::Unexpected(t) => match t {
                UnexpectedField::String(v) => state.serialize_field("value", &v)?,
                UnexpectedField::ConcealedString(v) => state.serialize_field("value", &v)?,
                UnexpectedField::WifiNetworkSecurityType(v) => {
                    state.serialize_field("value", &v)?
                }
                UnexpectedField::SubdivisionCode(v) => state.serialize_field("value", &v)?,
                UnexpectedField::CountryCode(v) => state.serialize_field("value", &v)?,
                UnexpectedField::Unknown { value: v, .. } => state.serialize_field("value", &v)?,
                UnexpectedField::Boolean(b) => {
                    let v = if b.0 { "true" } else { "false" };
                    state.serialize_field("value", v)?
                }
                UnexpectedField::Date(date) => state.serialize_field("value", &date)?,
                UnexpectedField::YearMonth(v) => state.serialize_field("value", &v)?,
                UnexpectedField::Email(v) => state.serialize_field("value", &v)?,
                UnexpectedField::Number(v) => state.serialize_field("value", &v)?,
            },
        }

        if let Some(ref label) = self.label {
            state.serialize_field("label", label)?;
        } else {
            state.skip_field("label")?;
        }

        if let Some(ref ext) = self.extensions {
            if ext.is_empty() {
                state.skip_field("extensions")?;
            } else {
                state.serialize_field("extensions", ext)?;
            }
        } else {
            state.skip_field("extensions")?;
        }

        state.end()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditableFieldHelper<E> {
    #[serde(default)]
    id: Option<B64Url>,
    value: String,
    field_type: FieldType,
    #[serde(default)]
    label: Option<String>,
    #[serde(default = "none::<E>")]
    extensions: Option<Vec<Extension<E>>>,
}

// Need to use this instead of the normal default,
// otherwise the derive creates a `E: Default` constraint.
fn none<E>() -> Option<Vec<Extension<E>>> {
    None
}

impl<'de, T, E> Deserialize<'de> for EditableField<T, E>
where
    T: EditableFieldType + DeserializeOwned,
    E: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper: EditableFieldHelper<E> = EditableFieldHelper::deserialize(deserializer)?;

        let value = if T::field_type() == helper.field_type {
            match T::deserialize(StrDeserializer::<serde::de::value::Error>::new(
                helper.value.as_str(),
            )) {
                Ok(t) => ExpectedInner::Expected(t),
                Err(_) => ExpectedInner::Unexpected(UnexpectedField::Unknown {
                    field_type: helper.field_type,
                    value: helper.value,
                }),
            }
        } else {
            macro_rules! deserialize_from_str {
                ($t:tt => $variant:ident) => {
                    match $t::deserialize(StrDeserializer::<D::Error>::new(&helper.value)) {
                        Ok(v) => UnexpectedField::$variant(v),
                        _ => UnexpectedField::Unknown {
                            field_type: helper.field_type,
                            value: helper.value,
                        },
                    }
                };
            }

            let value = match helper.field_type {
                FieldType::String => UnexpectedField::String(EditableFieldString(helper.value)),
                FieldType::ConcealedString => {
                    UnexpectedField::ConcealedString(EditableFieldConcealedString(helper.value))
                }
                FieldType::Boolean => deserialize_from_str!(EditableFieldBoolean => Boolean),
                FieldType::Date => deserialize_from_str!(EditableFieldDate => Date),
                FieldType::YearMonth => deserialize_from_str!(EditableFieldYearMonth => YearMonth),
                FieldType::WifiNetworkSecurityType => {
                    deserialize_from_str!(EditableFieldWifiNetworkSecurityType => WifiNetworkSecurityType)
                }
                FieldType::SubdivisionCode => {
                    UnexpectedField::SubdivisionCode(EditableFieldSubdivisionCode(helper.value))
                }
                FieldType::CountryCode => {
                    UnexpectedField::CountryCode(EditableFieldCountryCode(helper.value))
                }
                FieldType::Email => UnexpectedField::Email(EditableFieldEmail(helper.value)),
                FieldType::Number => UnexpectedField::Number(EditableFieldNumber(helper.value)),
                FieldType::Unknown(_) => UnexpectedField::Unknown {
                    field_type: helper.field_type,
                    value: helper.value,
                },
            };

            ExpectedInner::Unexpected(value)
        };

        Ok(Self {
            id: helper.id,
            value: Expected(value),
            label: helper.label,
            extensions: helper.extensions,
        })
    }
}

// Helper for converting inner types into EditableField
impl<T, E> From<T> for EditableField<T, E> {
    fn from(s: T) -> Self {
        EditableField {
            id: None,
            value: s.into(),
            label: None,
            extensions: None,
        }
    }
}

/// Macro to define a string-backed `EditableField` type with its `EditableFieldType` impl
/// and `From` conversions for `String`.
macro_rules! editable_field_string_type {
    ($name:ident, $variant:ident) => {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl EditableFieldType for $name {
            fn field_type() -> FieldType {
                FieldType::$variant
            }
        }

        impl From<String> for $name {
            #[inline]
            fn from(s: String) -> Self {
                $name(s)
            }
        }

        impl<E> From<String> for EditableField<$name, E> {
            #[inline]
            fn from(s: String) -> Self {
                $name(s).into()
            }
        }

        impl<E> From<EditableField<$name, E>> for String {
            fn from(s: EditableField<$name, E>) -> Self {
                match s.value.0 {
                    ExpectedInner::Expected(f) => f.0.into(),
                    ExpectedInner::Unexpected(f) => f.into(),
                }
            }
        }

        impl From<$name> for String {
            #[inline]
            fn from(v: $name) -> Self {
                v.0
            }
        }

        impl AsRef<str> for $name {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

editable_field_string_type!(EditableFieldString, String);
editable_field_string_type!(EditableFieldConcealedString, ConcealedString);
editable_field_string_type!(EditableFieldEmail, Email);
editable_field_string_type!(EditableFieldNumber, Number);
editable_field_string_type!(EditableFieldSubdivisionCode, SubdivisionCode);
editable_field_string_type!(EditableFieldCountryCode, CountryCode);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldBoolean(#[serde(with = "serde_bool")] pub bool);
impl EditableFieldType for EditableFieldBoolean {
    fn field_type() -> FieldType {
        FieldType::Boolean
    }
}

impl<E> From<bool> for EditableField<EditableFieldBoolean, E> {
    fn from(b: bool) -> Self {
        EditableFieldBoolean(b).into()
    }
}

impl From<EditableFieldBoolean> for String {
    #[inline]
    fn from(value: EditableFieldBoolean) -> Self {
        if value.0 { "true" } else { "false" }.into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldDate(pub NaiveDate);
impl EditableFieldType for EditableFieldDate {
    fn field_type() -> FieldType {
        FieldType::Date
    }
}

impl From<EditableFieldDate> for String {
    #[inline]
    fn from(value: EditableFieldDate) -> Self {
        value.0.format("%Y-%m-%d").to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(into = "String")]
pub struct EditableFieldYearMonth {
    /// The year in the format `YYYY`
    pub year: u16,
    /// The month in the format `MM`
    pub month: Month,
}

impl From<EditableFieldYearMonth> for String {
    #[inline]
    fn from(value: EditableFieldYearMonth) -> String {
        format!("{:04}-{:02}", value.year, value.month.number_from_month())
    }
}

impl EditableFieldType for EditableFieldYearMonth {
    fn field_type() -> FieldType {
        FieldType::YearMonth
    }
}

impl<'de> Deserialize<'de> for EditableFieldYearMonth {
    fn deserialize<D>(deserializer: D) -> Result<EditableFieldYearMonth, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = deserializer.deserialize_str(CowVisitor)?;
        let (year_str, month_str) = s
            .split_once('-')
            .ok_or_else(|| serde::de::Error::custom("Invalid format"))?;

        Ok(EditableFieldYearMonth {
            year: year_str
                .parse::<u16>()
                .map_err(|_| serde::de::Error::custom("Invalid year"))?,
            month: month_str
                .parse::<u8>()
                .map_err(|_| serde::de::Error::custom("Invalid month"))?
                .try_into()
                .map_err(|_| serde::de::Error::custom("Invalid month"))?,
        })
    }
}

/// Deserialize strings into `Cow` to avoid unnecessary allocations
struct CowVisitor;
impl<'de> Visitor<'de> for CowVisitor {
    type Value = Cow<'de, str>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Cow::Borrowed(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Cow::Owned(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Cow::Owned(value))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum EditableFieldWifiNetworkSecurityType {
    Unsecured,
    WpaPersonal,
    Wpa2Personal,
    Wpa3Personal,
    Wep,

    #[serde(untagged)]
    Other(String),
}
impl EditableFieldType for EditableFieldWifiNetworkSecurityType {
    fn field_type() -> FieldType {
        FieldType::WifiNetworkSecurityType
    }
}

impl From<EditableFieldWifiNetworkSecurityType> for String {
    #[inline]
    fn from(value: EditableFieldWifiNetworkSecurityType) -> Self {
        match value {
            EditableFieldWifiNetworkSecurityType::Unsecured => "unsecured".into(),
            EditableFieldWifiNetworkSecurityType::WpaPersonal => "wpa-personal".into(),
            EditableFieldWifiNetworkSecurityType::Wpa2Personal => "wpa2-personal".into(),
            EditableFieldWifiNetworkSecurityType::Wpa3Personal => "wpa3-personal".into(),
            EditableFieldWifiNetworkSecurityType::Wep => "wep".into(),
            EditableFieldWifiNetworkSecurityType::Other(o) => o,
        }
    }
}

/// Helper wrapper for `CustomFieldsCredential`.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged, bound(deserialize = "E: Deserialize<'de>"))]
#[non_exhaustive]
pub enum EditableFieldValue<E = ()> {
    String(EditableField<EditableFieldString, E>),
    ConcealedString(EditableField<EditableFieldConcealedString, E>),
    Email(EditableField<EditableFieldEmail, E>),
    Number(EditableField<EditableFieldNumber, E>),
    Boolean(EditableField<EditableFieldBoolean, E>),
    Date(EditableField<EditableFieldDate, E>),
    YearMonth(EditableField<EditableFieldYearMonth, E>),
    SubdivisionCode(EditableField<EditableFieldSubdivisionCode, E>),
    CountryCode(EditableField<EditableFieldCountryCode, E>),
    WifiNetworkSecurityType(EditableField<EditableFieldWifiNetworkSecurityType, E>),
}

impl<'de, E> Deserialize<'de> for EditableFieldValue<E>
where
    E: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper: EditableFieldHelper<E> = EditableFieldHelper::deserialize(deserializer)?;

        macro_rules! deserialize_field {
            ($t: tt) => {{
                let v = $t::deserialize(StringDeserializer::new(helper.value))?;

                EditableField {
                    id: helper.id,
                    value: Expected(ExpectedInner::Expected(v)),
                    label: helper.label,
                    extensions: helper.extensions,
                }
            }};
        }

        let res = match helper.field_type {
            FieldType::String => Self::String(deserialize_field!(EditableFieldString)),
            FieldType::ConcealedString => {
                Self::ConcealedString(deserialize_field!(EditableFieldConcealedString))
            }
            FieldType::Boolean => Self::Boolean(deserialize_field!(EditableFieldBoolean)),
            FieldType::Date => Self::Date(deserialize_field!(EditableFieldDate)),
            FieldType::YearMonth => Self::YearMonth(deserialize_field!(EditableFieldYearMonth)),
            FieldType::WifiNetworkSecurityType => Self::WifiNetworkSecurityType(
                deserialize_field!(EditableFieldWifiNetworkSecurityType),
            ),
            FieldType::SubdivisionCode => {
                Self::SubdivisionCode(deserialize_field!(EditableFieldSubdivisionCode))
            }
            FieldType::CountryCode => {
                Self::CountryCode(deserialize_field!(EditableFieldCountryCode))
            }
            FieldType::Email => Self::Email(deserialize_field!(EditableFieldEmail)),
            FieldType::Number => Self::Number(deserialize_field!(EditableFieldNumber)),
            FieldType::Unknown(_) => {
                return Err(serde::de::Error::custom("Unknown custom field type"))
            }
        };

        Ok(res)
    }
}

mod serde_bool {
    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = deserializer.deserialize_str(super::CowVisitor)?;

        s.trim()
            .to_lowercase()
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize_editable_field_string() {
        let field: EditableField<EditableFieldString> = EditableField {
            id: None,
            value: EditableFieldString("value".to_string()).into(),
            label: Some("label".to_string()),
            extensions: None,
        };
        let json = json!({
            "value": "value",
            "fieldType": "string",
            "label": "label",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }

    #[test]
    fn test_deserialize_field_string() {
        let json = json!({
            "value": "value",
            "fieldType": "string",
            "label": "label",
        });
        let field: EditableField<EditableFieldString> = serde_json::from_value(json).unwrap();

        assert_eq!(
            field,
            EditableField {
                id: None,
                value: EditableFieldString("value".to_string()).into(),
                label: Some("label".to_string()),
                extensions: None,
            }
        );
    }

    #[test]
    fn test_serialize_field_concealed_string() {
        let field: EditableField<EditableFieldConcealedString> = EditableField {
            id: None,
            value: EditableFieldConcealedString("value".to_string()).into(),
            label: Some("label".to_string()),
            extensions: None,
        };
        let json = json!({
            "fieldType": "concealed-string",
            "value": "value",
            "label": "label",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }

    #[test]
    fn test_deserialize_field_wrong_type() {
        let json = json!({
            "value": "value",
            "fieldType": "string",
            "label": "label",
        });
        let field: Result<EditableField<EditableFieldConcealedString>, _> =
            serde_json::from_value(json);

        assert!(field.unwrap().value.as_expected().is_err());
    }

    #[test]
    fn test_deserialize_field_bad_value_string() {
        let json = json!({
            "value": 5,
            "fieldType": "string",
            "label": "label",
        });
        let field: Result<EditableField<EditableFieldString>, _> = serde_json::from_value(json);

        assert!(field.is_err());
    }

    #[test]
    fn test_deserialize_field_bad_value_bool() {
        let json = json!({
            "value": "bad",
            "fieldType": "bool",
            "label": "label",
        });
        let field: Result<EditableField<EditableFieldBoolean>, _> = serde_json::from_value(json);

        assert!(field.unwrap().value.as_expected().is_err());
    }

    #[test]
    fn test_deserialize_field_missing_type() {
        let json = json!({
            "value": "value",
            "label": "label",
        });
        let field: Result<EditableField<EditableFieldConcealedString>, _> =
            serde_json::from_value(json);

        assert!(field.is_err());
    }

    #[test]
    fn test_deserialize_field_concealed_string() {
        let json = json!({
            "value": "value",
            "fieldType": "concealed-string",
            "label": "label",
        });
        let field: EditableField<EditableFieldConcealedString> =
            serde_json::from_value(json).unwrap();

        assert_eq!(
            field,
            EditableField {
                id: None,
                value: EditableFieldConcealedString("value".to_string()).into(),
                label: Some("label".to_string()),
                extensions: None,
            }
        );
    }

    #[test]
    fn test_serialize_field_boolean() {
        let field: EditableField<EditableFieldBoolean> = EditableField {
            id: None,
            value: EditableFieldBoolean(true).into(),
            label: Some("label".to_string()),
            extensions: None,
        };
        let json = json!({
            "fieldType": "boolean",
            "value": "true",
            "label": "label",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }

    #[test]
    fn test_serialize_field_date() {
        let field: EditableField<EditableFieldDate> = EditableField {
            id: None,
            value: EditableFieldDate(NaiveDate::from_ymd_opt(2025, 2, 24).unwrap()).into(),
            label: None,
            extensions: None,
        };
        let json = json!({
            "fieldType": "date",
            "value": "2025-02-24",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }

    #[test]
    fn test_serialize_editable_field_year_month() {
        let field: EditableField<EditableFieldYearMonth> = EditableField {
            id: None,
            value: EditableFieldYearMonth {
                year: 2025,
                month: Month::February,
            }
            .into(),
            label: None,
            extensions: None,
        };
        let json = json!({
            "fieldType": "year-month",
            "value": "2025-02",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }

    #[test]
    fn test_deserialize_editable_field_year_month() {
        let json = json!({
            "fieldType": "year-month",
            "value": "2025-02",
        });
        let field: EditableField<EditableFieldYearMonth> = serde_json::from_value(json).unwrap();

        assert_eq!(
            field,
            EditableField {
                id: None,
                value: EditableFieldYearMonth {
                    year: 2025,
                    month: Month::February,
                }
                .into(),
                label: None,
                extensions: None,
            }
        );
    }

    #[test]
    fn test_deserialize_editable_field_year_month_invalid_format() {
        let json = json!({
            "fieldType": "year-month",
            "value": "2025/02",
        });
        let field: EditableField<EditableFieldYearMonth> = serde_json::from_value(json).unwrap();
        assert_eq!(
            field.value.as_expected(),
            Err(&UnexpectedField::Unknown {
                field_type: FieldType::YearMonth,
                value: "2025/02".into(),
            })
        );
    }

    #[test]
    fn test_extension_round_trip() {
        let json = json!({
            "fieldType": "string",
            "value": "hello",
            "extensions": [
                {
                    "name": "test",
                    "contents": "world"
                }
            ]
        });
        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        #[serde(tag = "name", rename_all = "camelCase")]
        enum CustomExtension {
            Test { contents: String },
        }

        let field: EditableField<EditableFieldString, CustomExtension> =
            serde_json::from_value(json.clone()).expect("Could not deserialize custom extensions");

        assert_eq!(
            field,
            EditableField {
                id: None,
                value: EditableFieldString("hello".to_string()).into(),
                label: None,
                extensions: Some(vec![Extension::External(CustomExtension::Test {
                    contents: "world".into()
                })])
            }
        );

        let returned = serde_json::to_value(field).expect("Could not serialize custom extensions");

        assert_eq!(returned, json);
    }

    #[test]
    fn editable_string_deserialized_from_others() {
        for (field_type, expected) in [
            (
                "concealed-string",
                UnexpectedField::ConcealedString("hello".to_owned().into()),
            ),
            (
                "unknown",
                UnexpectedField::Unknown {
                    field_type: FieldType::Unknown("unknown".into()),
                    value: "hello".into(),
                },
            ),
            (
                "date",
                UnexpectedField::Unknown {
                    field_type: FieldType::Date,
                    value: "hello".into(),
                },
            ),
        ] {
            let json = json!({
                "fieldType": field_type,
                "value": "hello",
            });

            let field: EditableField<EditableFieldString> =
                serde_json::from_value(json.clone()).expect("Could not deserialize field");

            assert_eq!(field.value.into_expected(), Err(expected));
        }
    }
}
