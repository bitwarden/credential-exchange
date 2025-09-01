//! # Document Credentials

use serde::{Deserialize, Serialize};

use crate::{B64Url, EditableField, EditableFieldString, EditableFieldValue, Extension};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct CustomFieldsCredential<E = ()> {
    /// A unique identifier for the CustomFields. It MUST be a machine-generated opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<B64Url>,
    /// This member is a [human-palatable](https://www.w3.org/TR/webauthn-3/#human-palatability)
    /// title to describe the section. This value MAY be set by the credential owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The collection of miscellaneous fields under this section.
    pub fields: Vec<EditableFieldValue<E>>,
    /// This member permits the exporting provider to add additional information associated to this
    /// CustomFields. This MAY be used to provide an exchange where a minimal amount of information
    /// is lost.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<Extension<E>>,
}

/// A [FileCredential] acts as a placeholder to an arbitrary binary file holding its associated
/// metadata. When an importing provider encounters a file credential, they MAY request the file
/// afterwards if they have a direct exchange. If the exchange will produce an export response file,
/// then the associated encrypted file MUST be stored in the documents folder of the zip archive.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCredential {
    /// The file’s identifier, used as the file name in the zip archive.
    pub id: B64Url,
    /// The file name with the file extension if applicable.
    pub name: String,
    /// The file’s decrypted size in bytes.
    pub decrypted_size: u64,
    /// The SHA256 hash of the decrypted file. This hash MUST be used by the importing provider
    /// when the file is decrypted to ensure that it has not been corrupted.
    pub integrity_hash: B64Url,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct NoteCredential<E = ()> {
    /// This member is a user-defined value encoded as a UTF-8 string.
    pub content: EditableField<EditableFieldString, E>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{EditableFieldBoolean, EditableFieldString};

    #[test]
    fn test_serialize_custom_fields() {
        let credential = CustomFieldsCredential {
            id: None,
            label: None,
            fields: vec![
                EditableFieldValue::<()>::String(EditableField {
                    id: Some(B64Url::from(b"field1".as_slice())),
                    value: EditableFieldString("hello".into()),
                    label: None,
                    extensions: None,
                }),
                EditableFieldValue::<()>::Boolean(EditableField {
                    id: None,
                    value: EditableFieldBoolean(false),
                    label: None,
                    extensions: None,
                }),
            ],
            extensions: vec![],
        };

        let json = json!({
            "fields": [
                {
                    "id": "ZmllbGQx",
                    "fieldType": "string",
                    "value": "hello"
                },
                {
                    "fieldType": "boolean",
                    "value": "false"
                }
            ]
        });

        assert_eq!(serde_json::to_value(&credential).unwrap(), json);
    }

    #[test]
    fn test_deserialize_custom_fields() {
        let json = json!({
            "fields": [
                {
                    "fieldType": "string",
                    "value": "hello"
                },
                {
                    "fieldType": "boolean",
                    "value": "false"
                }
            ]
        });

        let json = serde_json::to_string(&json).unwrap();
        let credential: CustomFieldsCredential = serde_json::from_str(&json).unwrap();

        assert_eq!(credential.id, None);
        assert_eq!(credential.label, None);
        assert_eq!(credential.extensions.len(), 0);
        assert_eq!(credential.fields.len(), 2);

        match &credential.fields[0] {
            EditableFieldValue::String(field) => {
                assert_eq!(field.value.0, "hello");
            }
            _ => panic!("Expected string field"),
        }

        match &credential.fields[1] {
            EditableFieldValue::Boolean(field) => {
                assert!(!field.value.0);
            }
            _ => panic!("Expected boolean field"),
        }
    }
}
