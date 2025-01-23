use serde::{de::DeserializeOwned, ser::SerializeStruct, Deserialize, Serialize};

use crate::B64Url;

#[derive(Clone, Debug)]
pub struct EditableField<T: EditableFieldType + Serialize + DeserializeOwned> {
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

impl<T> Serialize for EditableField<T>
where
    T: EditableFieldType + Serialize + DeserializeOwned,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("editable_field", 4)?;
        if let Some(ref id) = self.id {
            state.serialize_field("id", id)?;
        }
        state.serialize_field("value", &self.value)?;
        state.serialize_field("field_type", &self.value.field_type())?;
        if let Some(ref label) = self.label {
            state.serialize_field("label", label)?;
        }
        state.end()
    }
}

impl<'de, T> Deserialize<'de> for EditableField<T>
where
    T: EditableFieldType + Serialize + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EditableFieldHelper<T> {
            id: Option<B64Url>,
            value: T,
            field_type: String,
            label: Option<String>,
        }

        let helper = EditableFieldHelper::deserialize(deserializer)?;
        Ok(Self {
            id: helper.id,
            value: helper.value,
            label: helper.label,
        })
    }
}

pub trait EditableFieldType<T: Serialize + DeserializeOwned = Self> {
    fn field_type(&self) -> &str;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditableFieldString(String);
impl EditableFieldType for EditableFieldString {
    fn field_type(&self) -> &str {
        "string"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditableFieldConcealedString(String);
impl EditableFieldType for EditableFieldConcealedString {
    fn field_type(&self) -> &str {
        "concealed-string"
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
    fn field_type(&self) -> &str {
        "boolean"
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
    let value = String::deserialize(deserializer)?;
    match value.as_str() {
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
    fn test_serialize_editable_field_concealed_string() {
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
    fn test_serialize_editable_field_boolean() {
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
