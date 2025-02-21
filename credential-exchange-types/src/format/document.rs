//! # Document Credentials

use serde::{Deserialize, Serialize};

use crate::{
    format::{EditableField, EditableFieldString, Extension},
    B64Url,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fields: Vec<EditableField<EditableFieldString, E>>,
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
    pub integration_hash: B64Url,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct NoteCredential<E = ()> {
    /// This member is a user-defined value encoded as a UTF-8 string.
    pub content: EditableField<EditableFieldString, E>,
}
