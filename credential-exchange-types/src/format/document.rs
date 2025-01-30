//! # Document Credentials

use serde::{Deserialize, Serialize};

use crate::B64Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCredential {
    /// This member is a user-defined value encoded as a UTF-8 string.
    pub content: String,
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
