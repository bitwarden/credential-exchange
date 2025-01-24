use serde::{de::DeserializeOwned, ser::SerializeStruct, Deserialize, Serialize};

use crate::B64Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditableField<T> {
    /// A unique identifier for the [EditableField] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: Option<B64Url>,
    /// This member defines the meaning of the [value][EditableField::value] member and its type.
    /// This meaning is two-fold:
    ///
    /// 1. The string representation of the value if its native type is not a string.
    /// 2. The UI representation used to display the value.
    ///
    /// The value SHOULD be a member of [FieldType] and the
    /// [importing provider](https://fidoalliance.org/specs/cx/cxp-v1.0-wd-20241003.html#importing-provider)
    /// SHOULD ignore any unknown values and default to [string][FieldType::String].
    /// pub field_type: FieldType,
    /// This member contains the [fieldType][EditableField::field_type] defined by the user.
    pub value: T,
    /// This member contains a user facing value describing the value stored. This value MAY be
    /// user defined.
    pub label: Option<String>,
}

/// Internal enum to represent the different field types.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum FieldType {
    String,
    ConcealedString,
    Boolean,
    Date,
    YearMonth,
    SubdivisionCode,
    CountryCode,

    #[serde(other)]
    Unknown,
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

        state.serialize_field("field_type", &self.value.field_type())?;
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
pub struct EditableFieldString(String);
impl EditableFieldType for EditableFieldString {
    fn field_type(&self) -> FieldType {
        FieldType::String
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditableFieldConcealedString(String);
impl EditableFieldType for EditableFieldConcealedString {
    fn field_type(&self) -> FieldType {
        FieldType::ConcealedString
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct EditableFieldBoolean(
    #[serde(
        serialize_with = "serialize_bool",
        deserialize_with = "deserialize_bool"
    )]
    bool,
);
impl EditableFieldType for EditableFieldBoolean {
    fn field_type(&self) -> FieldType {
        FieldType::Boolean
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditableFieldDate(String);
impl EditableFieldType for EditableFieldDate {
    fn field_type(&self) -> FieldType {
        FieldType::Date
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditableFieldYearMonth(String);
impl EditableFieldType for EditableFieldYearMonth {
    fn field_type(&self) -> FieldType {
        FieldType::YearMonth
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditableFieldSubdivisionCode(String);
impl EditableFieldType for EditableFieldSubdivisionCode {
    fn field_type(&self) -> FieldType {
        FieldType::SubdivisionCode
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EditableFieldCountryCode(String);
impl EditableFieldType for EditableFieldCountryCode {
    fn field_type(&self) -> FieldType {
        FieldType::CountryCode
    }
}

fn serialize_bool<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(if *value { "true" } else { "false" })
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <&str>::deserialize(deserializer)?;
    match value.trim().to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::custom("expected 'true' or 'false'")),
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
            "field_type": "string",
            "label": "label",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }

    #[test]
    fn test_deserialize_field_string() {
        let json = json!({
            "value": "value",
            "field_type": "string",
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
            "field_type": "concealed-string",
            "value": "value",
            "label": "label",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }

    #[test]
    fn test_deserialize_field_wrong_type() {
        let json = json!({
            "value": "value",
            "field_type": "string",
            "label": "label",
        });
        let field: Result<EditableField<EditableFieldConcealedString>, _> =
            serde_json::from_value(json);

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
            "field_type": "concealed-string",
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
            "field_type": "boolean",
            "value": "true",
            "label": "label",
        });
        assert_eq!(serde_json::to_value(&field).unwrap(), json);
    }
}
