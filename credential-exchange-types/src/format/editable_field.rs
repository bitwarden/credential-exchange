use serde::{de::DeserializeOwned, ser::SerializeStruct, Deserialize, Serialize};

use crate::B64Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditableField<T> {
    /// A unique identifier for the [EditableField] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: Option<B64Url>,
    /// This member contains the fieldType defined by the user.
    pub value: T,
    /// This member contains a user facing value describing the value stored. This value MAY be
    /// user defined.
    pub label: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
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
    /// This is equivalent to the YYYY-MM format specified in ISO8601.
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
trait EditableFieldType {
    /// The `field_type` value associated with the type
    fn field_type(&self) -> FieldType;
}

impl<T> Serialize for EditableField<T>
where
    T: EditableFieldType + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 2 + self.id.is_some() as usize + self.label.is_some() as usize;
        let mut state = serializer.serialize_struct("editable_field", len)?;

        if let Some(ref id) = self.id {
            state.serialize_field("id", id)?;
        } else {
            state.skip_field("id")?;
        }

        state.serialize_field("fieldType", &self.value.field_type())?;
        state.serialize_field("value", &self.value)?;

        if let Some(ref label) = self.label {
            state.serialize_field("label", label)?;
        } else {
            state.skip_field("label")?;
        }

        state.end()
    }
}

impl<'de, T> Deserialize<'de> for EditableField<T>
where
    T: EditableFieldType + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct EditableFieldHelper<T> {
            id: Option<B64Url>,
            value: T,
            field_type: FieldType,
            label: Option<String>,
        }

        let helper: EditableFieldHelper<T> = EditableFieldHelper::deserialize(deserializer)?;

        if helper.field_type != helper.value.field_type() {
            return Err(serde::de::Error::custom(
                "field_type does not match value type",
            ));
        }

        Ok(Self {
            id: helper.id,
            value: helper.value,
            label: helper.label,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldString(pub String);
impl EditableFieldType for EditableFieldString {
    fn field_type(&self) -> FieldType {
        FieldType::String
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldConcealedString(pub String);
impl EditableFieldType for EditableFieldConcealedString {
    fn field_type(&self) -> FieldType {
        FieldType::ConcealedString
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditableFieldBoolean(#[serde(with = "serde_bool")] pub bool);
impl EditableFieldType for EditableFieldBoolean {
    fn field_type(&self) -> FieldType {
        FieldType::Boolean
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldDate(pub String);
impl EditableFieldType for EditableFieldDate {
    fn field_type(&self) -> FieldType {
        FieldType::Date
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldYearMonth(pub String);
impl EditableFieldType for EditableFieldYearMonth {
    fn field_type(&self) -> FieldType {
        FieldType::YearMonth
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldSubdivisionCode(pub String);
impl EditableFieldType for EditableFieldSubdivisionCode {
    fn field_type(&self) -> FieldType {
        FieldType::SubdivisionCode
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct EditableFieldCountryCode(pub String);
impl EditableFieldType for EditableFieldCountryCode {
    fn field_type(&self) -> FieldType {
        FieldType::CountryCode
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
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
    fn field_type(&self) -> FieldType {
        FieldType::WifiNetworkSecurityType
    }
}

mod serde_bool {
    use serde::Deserialize;

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
        let value = <&str>::deserialize(deserializer)?;

        value
            .trim()
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
        let field = EditableField {
            id: None,
            value: EditableFieldString("value".to_string()),
            label: Some("label".to_string()),
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
                value: EditableFieldString("value".to_string()),
                label: Some("label".to_string()),
            }
        );
    }

    #[test]
    fn test_serialize_field_concealed_string() {
        let field = EditableField {
            id: None,
            value: EditableFieldConcealedString("value".to_string()),
            label: Some("label".to_string()),
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

        assert!(field.is_err());
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

        assert!(field.is_err());
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
                value: EditableFieldConcealedString("value".to_string()),
                label: Some("label".to_string()),
            }
        );
    }

    #[test]
    fn test_serialize_field_boolean() {
        let field = EditableField {
            id: None,
            value: EditableFieldBoolean(true),
            label: Some("label".to_string()),
        };
        let json = json!({
            "fieldType": "boolean",
            "value": "true",
            "label": "label",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }
}
